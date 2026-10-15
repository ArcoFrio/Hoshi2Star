//! Characterisation probes for the 2026-09-05 audit.
//! These assertions document CURRENT defects, not desired behaviour.
use hoshi2star_lib::{
    core::qa,
    db,
    engines::mv_mz::{extractor, injector},
    llm::{
        pipeline,
        provider::{LlmError, LlmProvider, PromptContextPolicy, TranslationContext},
        tokenizer::{Engine, Tokenizer},
    },
};
use serde_json::json;

fn context() -> TranslationContext {
    TranslationContext {
        source_lang: "ja".into(),
        target_lang: "fr".into(),
        glossary_terms: vec![],
        engine: "mv_mz".into(),
        batch_size: 2,
        batch_delay_ms: 0,
        prompt_context_policy: PromptContextPolicy::EngineOwned,
        segment_contexts: vec![],
    }
}

#[test]
fn bare_system_markers_are_unprotected_and_qa_accepts_their_loss() {
    let source = "%1 obtained %2!";
    let tokenized = Tokenizer::tokenize(source, Engine::from_project_engine("mv_mz"));
    assert!(tokenized.map.is_empty());
    let result = qa::check(source, "Objet obtenu !", &[], "mv_mz");
    assert_eq!(result.score, 100);
    assert!(!result.has_critical_errors());
}

#[test]
fn visible_map_and_actor_change_fields_are_not_extracted() {
    let map = json!({"displayName":"Village", "events":[null,{"pages":[{"list":[
        {"code":320,"parameters":[1,"Alice"]},
        {"code":324,"parameters":[1,"Mage"]},
        {"code":325,"parameters":[1,"A travelling mage."]},
        {"code":401,"parameters":["Hello!"]}
    ]}]}]});
    let segments = extractor::extract_map(&map);
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].source, "Hello!");
    let system = json!({"gameTitle":"Demo", "equipTypes":["","Shield"], "skillTypes":["","Magic"]});
    assert_eq!(extractor::extract_system(&system).len(), 1);
}

#[test]
fn legitimate_shared_vocabulary_is_a_critical_error() {
    let neighbors = vec![];
    let french = qa::QaSemanticContext {
        source_language: "fr",
        target_language: "en",
        segment_kind: "item_name",
        neighbor_sources: &neighbors,
    };
    let result = qa::check_with_context("Potion", "Potion", &[], "mv_mz", &french);
    assert!(result.has_critical_errors());
    let japanese = qa::QaSemanticContext {
        source_language: "ja",
        target_language: "zh",
        segment_kind: "system_term",
        neighbor_sources: &neighbors,
    };
    let result = qa::check_with_context("魔法", "魔法", &[], "mv_mz", &japanese);
    assert!(result.has_critical_errors());
}

#[test]
fn unknown_tokens_and_reordered_color_codes_are_accepted() {
    let tokenized = Tokenizer::tokenize(r"\C[2]Bonjour\C[0]", Engine::MvMz);
    let restored = Tokenizer::restore("⟦ph_1⟧Hello⟦ph_0⟧ ⟦ph_99⟧", &tokenized.map).unwrap();
    assert_eq!(restored, r"\C[0]Hello\C[2] ⟦ph_99⟧");
    assert_eq!(
        qa::check(r"\C[2]Bonjour\C[0]", &restored, &[], "mv_mz").score,
        100
    );
}

#[test]
fn injection_does_not_verify_the_original_text() {
    let mut changed_game = json!([null,{"name":"New item after game update"}]);
    injector::inject(&mut changed_game, &[("/1/name", "Old translated item")]).unwrap();
    assert_eq!(changed_game[1]["name"], "Old translated item");
}

struct FailingProvider;
impl LlmProvider for FailingProvider {
    async fn translate(
        &self,
        _: Vec<String>,
        _: TranslationContext,
    ) -> Result<Vec<String>, LlmError> {
        Err(LlmError::Unavailable {
            message: "HTTP 401".into(),
        })
    }
    async fn health_check(&self) -> Result<(), LlmError> {
        Ok(())
    }
    async fn chat(&self, _: &str, _: &str) -> Result<String, LlmError> {
        unreachable!()
    }
}

#[tokio::test]
async fn provider_failure_becomes_successful_pipeline_and_is_excluded_from_resume() {
    let temp = tempfile::NamedTempFile::new().unwrap();
    let pool = db::pool::init(temp.path().to_str().unwrap()).await.unwrap();
    sqlx::query("INSERT INTO projects(id,name,engine,game_path) VALUES('p','Demo','mv_mz','/tmp')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO source_files(id,project_id,file_name,file_path,file_type) VALUES('f','p','Map001.json','/tmp/Map001.json','map')").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO segments(id,source_file_id,json_key,source_text,segment_kind) VALUES('s','f','/text','こんにちは','dialogue')").execute(&pool).await.unwrap();
    let result = pipeline::run_inner(
        vec![("s".into(), "こんにちは".into())],
        &FailingProvider,
        context(),
        &pool,
        None,
        None,
        None,
        |_, _| {},
    )
    .await
    .unwrap();
    assert!(result[0].needs_review);
    let resumable: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM segments WHERE status = 'untranslated' OR target_text = ''",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(resumable, 0);
}

#[tokio::test]
async fn qa_context_query_exceeds_sqlite_variable_limit_on_large_projects() {
    let temp = tempfile::NamedTempFile::new().unwrap();
    let pool = db::pool::init(temp.path().to_str().unwrap()).await.unwrap();
    let ids = (0..33000).map(|n| n.to_string()).collect::<Vec<_>>();
    let result = hoshi2star_lib::llm::context::build_for_segments(&pool, "mv_mz", &ids).await;
    let error = result.unwrap_err().to_string();
    assert!(error.contains("too many SQL variables"), "{error}");
}

#[tokio::test]
async fn reopening_preserves_missing_context_and_original_language_pair() {
    use hoshi2star_lib::{commands::project::open_project, state::AppState};
    use tauri::{
        test::{mock_builder, mock_context, noop_assets},
        Manager,
    };
    let temp = tempfile::tempdir().unwrap();
    let game = temp.path().join("game");
    std::fs::create_dir_all(game.join("data")).unwrap();
    std::fs::write(game.join("data/System.json"), r#"{"gameTitle":"Demo"}"#).unwrap();
    std::fs::write(
        game.join("data/Map001.json"),
        r#"{"events":[null,{"pages":[{"list":[{"code":401,"parameters":["Hello"]}]}]}]}"#,
    )
    .unwrap();
    let pool = db::pool::init(temp.path().join("db.sqlite").to_str().unwrap())
        .await
        .unwrap();
    let app = mock_builder()
        .manage(AppState { db: pool.clone() })
        .build(mock_context(noop_assets()))
        .unwrap();
    let path = game.to_string_lossy().into_owned();
    let first = open_project(
        path.clone(),
        Some("ja".into()),
        Some("fr".into()),
        app.state(),
    )
    .await
    .unwrap();
    // State resulting from the defaults of migration 0007 on an old project.
    sqlx::query("UPDATE segments SET segment_kind='unknown',scene_id=NULL,sequence_index=NULL,speaker=NULL,branch_path=NULL,context_json=NULL").execute(&pool).await.unwrap();
    let restored = open_project(path, Some("fr".into()), Some("en".into()), app.state())
        .await
        .unwrap();
    assert!(restored.was_restored);
    assert_eq!(restored.project.id, first.project.id);
    assert_eq!(restored.project.target_lang, "fr");
    let populated: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM segments WHERE segment_kind != 'unknown'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(populated, 0);
}

#[tokio::test]
async fn font_export_changes_mz_speaker_name_after_the_qa_gate() {
    use hoshi2star_lib::{
        commands::{
            export::export_project,
            project::{open_project, update_segment},
        },
        state::AppState,
    };
    use std::io::Read;
    use tauri::{
        test::{mock_builder, mock_context, noop_assets},
        Manager,
    };
    let temp = tempfile::tempdir().unwrap();
    let game = temp.path().join("game");
    std::fs::create_dir_all(game.join("data")).unwrap();
    std::fs::write(game.join("data/System.json"), r#"{"gameTitle":"デモ"}"#).unwrap();
    std::fs::write(game.join("data/Map001.json"), r#"{"events":[null,{"pages":[{"list":[{"code":101,"parameters":["",0,0,2,"アリス"]},{"code":401,"parameters":["こんにちは"]}]}]}]}"#).unwrap();
    let pool = db::pool::init(temp.path().join("db.sqlite").to_str().unwrap())
        .await
        .unwrap();
    let app = mock_builder()
        .manage(AppState { db: pool.clone() })
        .build(mock_context(noop_assets()))
        .unwrap();
    let project = open_project(
        game.to_string_lossy().into_owned(),
        Some("ja".into()),
        Some("fr".into()),
        app.state(),
    )
    .await
    .unwrap()
    .project;
    let rows: Vec<(String, String)> = sqlx::query_as("SELECT id,segment_kind FROM segments")
        .fetch_all(&pool)
        .await
        .unwrap();
    for (id, kind) in rows {
        let text = match kind.as_str() {
            "speaker" => "Alice",
            "game_title" => "Demo",
            _ => "Bonjour",
        };
        update_segment(id, text.into(), app.state()).await.unwrap();
    }
    let zip_path = export_project(project.id, Some(18), false, app.state())
        .await
        .unwrap();
    let mut archive = zip::ZipArchive::new(std::fs::File::open(zip_path).unwrap()).unwrap();
    let mut bytes = String::new();
    archive
        .by_name("data/Map001.json")
        .unwrap()
        .read_to_string(&mut bytes)
        .unwrap();
    let json: serde_json::Value = serde_json::from_str(&bytes).unwrap();
    assert_eq!(
        json.pointer("/events/1/pages/0/list/0/parameters/4")
            .unwrap(),
        r"\FS[18]Alice"
    );
}

#[tokio::test]
async fn inspect_fixture_extraction_counts_without_outputting_game_text() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../test");
    let mut paths = std::fs::read_dir(root)
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.join("data/System.json").exists() || p.join("www/data/System.json").exists())
        .collect::<Vec<_>>();
    paths.sort();
    for (i, path) in paths.iter().enumerate() {
        let isolated = tempfile::tempdir().unwrap();
        let source_data = if path.join("data").is_dir() {
            path.join("data")
        } else {
            path.join("www/data")
        };
        let isolated_data = isolated.path().join("data");
        std::fs::create_dir(&isolated_data).unwrap();
        for entry in std::fs::read_dir(source_data)
            .unwrap()
            .filter_map(Result::ok)
        {
            if entry.path().extension().is_some_and(|e| e == "json") {
                std::fs::copy(entry.path(), isolated_data.join(entry.file_name())).unwrap();
            }
        }
        let dump_path = hoshi2star_lib::commands::project::debug_dump_segments(
            isolated.path().to_string_lossy().into_owned(),
        )
        .await
        .unwrap();
        let value: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(dump_path).unwrap()).unwrap();
        println!(
            "FIXTURE {}: files={}, segments={}",
            i + 1,
            value["total_files"],
            value["total_segments"]
        );
    }
}
