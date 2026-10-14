//! Optional text categories (MV/MZ): scan → decide → translate → export.
//!
//! Uses the synthetic `mz_coverage` fixture, which deliberately contains a
//! note tag that looks a skill type up by name, an enabled plugin parameter
//! that mentions an element name, a disabled plugin, a script comparing an
//! actor name and a map display name.

use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};

use hoshi2star_lib::commands::coverage::{
    scan_text_coverage, set_text_categories, CategoryChange, CoverageView,
};
use hoshi2star_lib::commands::export::export_project;
use hoshi2star_lib::commands::project::{
    get_segments, get_source_files, open_project, update_segment,
};
use hoshi2star_lib::db;
use hoshi2star_lib::engines::mv_mz::coverage::CategoryState;
use hoshi2star_lib::state::AppState;
use serde_json::Value;
use tauri::test::{mock_builder, mock_context, noop_assets};
use tauri::Manager;

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mv_mz/mz_coverage")
}

fn copy_tree(source: &Path, destination: &Path) {
    std::fs::create_dir_all(destination).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

struct Opened {
    _temporary: tempfile::TempDir,
    app: tauri::App<tauri::test::MockRuntime>,
    project_id: String,
}

async fn opened() -> Opened {
    let temporary = tempfile::tempdir().unwrap();
    let game = temporary.path().join("game");
    copy_tree(&fixture(), &game);
    let db_path = temporary.path().join("h2s.db");
    let pool = db::pool::init(db_path.to_str().unwrap()).await.unwrap();
    let app = mock_builder()
        .manage(AppState::new(pool, temporary.path().join("app-data")).unwrap())
        .build(mock_context(noop_assets()))
        .unwrap();
    let project_id = open_project(
        game.to_string_lossy().into_owned(),
        Some("ja".into()),
        Some("en".into()),
        app.state(),
    )
    .await
    .expect("open fixture")
    .project
    .id;
    Opened {
        _temporary: temporary,
        app,
        project_id,
    }
}

fn state_of(view: &CoverageView, key: &str) -> CategoryState {
    view.categories
        .iter()
        .find(|c| c.key == key)
        .unwrap_or_else(|| panic!("missing category {key}"))
        .state
}

fn change(key: &str, enabled: bool) -> CategoryChange {
    CategoryChange {
        category_key: key.into(),
        enabled,
        accept_consequences: false,
    }
}

/// `(json_key, source, text_category)` of every segment, by file name.
async fn segments_by_file(
    opened: &Opened,
) -> BTreeMap<String, Vec<(String, String, String, Option<String>)>> {
    let mut out = BTreeMap::new();
    for file in get_source_files(opened.project_id.clone(), opened.app.state())
        .await
        .unwrap()
    {
        let page = get_segments(
            opened.project_id.clone(),
            file.id.clone(),
            0,
            100_000,
            opened.app.state(),
        )
        .await
        .unwrap();
        out.insert(
            file.file_name.clone(),
            page.items
                .into_iter()
                .map(|s| (s.id, s.json_key, s.source_text, s.text_category))
                .collect(),
        );
    }
    out
}

fn zip_json(zip_path: &str, entry: &str) -> Value {
    let mut archive = zip::ZipArchive::new(std::fs::File::open(zip_path).unwrap()).unwrap();
    let mut bytes = Vec::new();
    archive
        .by_name(entry)
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn scan_classifies_categories_by_reference_safety() {
    let opened = opened().await;
    let view = scan_text_coverage(opened.project_id.clone(), opened.app.state())
        .await
        .unwrap();

    assert!(view.supported);
    assert!(!view.has_decisions);
    assert!(view.plugins_unreadable.is_none());
    // Looked up by name in a Weapons note tag.
    assert_eq!(
        state_of(&view, "system.skill_types"),
        CategoryState::Blocked
    );
    // Mentioned by an enabled plugin's parameter (the disabled plugin and the
    // editor-only comment are not references).
    assert_eq!(state_of(&view, "system.elements"), CategoryState::Blocked);
    let elements = view
        .categories
        .iter()
        .find(|c| c.key == "system.elements")
        .unwrap();
    assert_eq!(elements.reference_count, 1);
    assert_eq!(
        elements.references[0].location,
        "Sample/MemoPlugin::PictureList.0.Name"
    );
    assert_eq!(
        state_of(&view, "system.equip_types"),
        CategoryState::Verified
    );
    assert_eq!(
        state_of(&view, "system.armor_types"),
        CategoryState::Verified
    );
    assert_eq!(
        state_of(&view, "system.weapon_types"),
        CategoryState::Verified
    );
    assert_eq!(state_of(&view, "map.display_name"), CategoryState::Verified);
    // The actor name is compared in a script: a non-blocking warning.
    assert!(view.core_warnings.iter().any(|w| w.source == "恵"));
    // Scanning never inserts segments.
    let segments = segments_by_file(&opened).await;
    assert!(segments.values().flatten().all(|s| s.3.is_none()));
}

#[tokio::test]
async fn blocked_categories_cannot_be_enabled() {
    let opened = opened().await;
    let error = set_text_categories(
        opened.project_id.clone(),
        vec![change("system.skill_types", true)],
        opened.app.state(),
    )
    .await
    .unwrap_err();
    assert!(error.contains("cannot be enabled"), "{error}");
    let segments = segments_by_file(&opened).await;
    assert!(segments.values().flatten().all(|s| s.3.is_none()));
}

#[tokio::test]
async fn enabled_categories_translate_export_and_park_round_trip() {
    let opened = opened().await;
    let view = set_text_categories(
        opened.project_id.clone(),
        vec![
            change("system.equip_types", true),
            change("map.display_name", true),
        ],
        opened.app.state(),
    )
    .await
    .unwrap();
    assert!(view.has_decisions);
    assert!(
        view.categories
            .iter()
            .find(|c| c.key == "map.display_name")
            .unwrap()
            .enabled
    );

    let segments = segments_by_file(&opened).await;
    let system: Vec<_> = segments["System.json"]
        .iter()
        .filter(|s| s.3.as_deref() == Some("system.equip_types"))
        .collect();
    assert_eq!(
        system.iter().map(|s| s.1.as_str()).collect::<Vec<_>>(),
        vec!["/equipTypes/1", "/equipTypes/2"]
    );
    assert!(segments["Map001.json"]
        .iter()
        .any(|s| s.1 == "/displayName"));

    // Translate everything that the export audit will see.
    for (id, _, source, _) in segments.values().flatten() {
        let target = match source.as_str() {
            "武器" => "Weapon".to_string(),
            "衣装" => "Outfit".to_string(),
            "始まりの村" => "Starting Village".to_string(),
            "%1は%2のダメージを受けた！" => "%1 took %2 damage!".to_string(),
            other => format!("EN[{}]", other.chars().count()),
        };
        update_segment(id.clone(), target, opened.app.state())
            .await
            .unwrap();
    }

    let zip = export_project(
        opened.project_id.clone(),
        Some(22),
        true,
        opened.app.state(),
    )
    .await
    .expect("export");
    let system_json = zip_json(&zip, "data/System.json");
    assert_eq!(
        system_json["equipTypes"],
        serde_json::json!(["", "Weapon", "Outfit"])
    );
    // Untouched lists stay exactly as they were.
    assert_eq!(
        system_json["skillTypes"],
        serde_json::json!(["", "魔術", "鞭"])
    );
    let map = zip_json(&zip, "data/Map001.json");
    // The map name popup draws plain text: never font-prefixed.
    assert_eq!(map["displayName"], "Starting Village");
    assert!(map["events"][1]["pages"][0]["list"][1]["parameters"][0]
        .as_str()
        .unwrap()
        .starts_with(r"\FS[22]"));

    // Disabling parks the translation; re-enabling restores it.
    set_text_categories(
        opened.project_id.clone(),
        vec![change("map.display_name", false)],
        opened.app.state(),
    )
    .await
    .unwrap();
    let after_disable = segments_by_file(&opened).await;
    assert!(after_disable["Map001.json"]
        .iter()
        .all(|s| s.1 != "/displayName"));
    set_text_categories(
        opened.project_id.clone(),
        vec![change("map.display_name", true)],
        opened.app.state(),
    )
    .await
    .unwrap();
    let restored = get_segments(
        opened.project_id.clone(),
        get_source_files(opened.project_id.clone(), opened.app.state())
            .await
            .unwrap()
            .into_iter()
            .find(|f| f.file_name == "Map001.json")
            .unwrap()
            .id,
        0,
        100,
        opened.app.state(),
    )
    .await
    .unwrap()
    .items
    .into_iter()
    .find(|s| s.json_key == "/displayName")
    .unwrap();
    assert_eq!(restored.target_text, "Starting Village");
    assert_eq!(restored.text_category.as_deref(), Some("map.display_name"));
}
