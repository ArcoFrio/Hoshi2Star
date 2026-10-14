//! Project-level orchestration of pristine game-file snapshots.
//!
//! The snapshot format and reconstruction rules are engine-owned
//! (`engines::mv_mz::originals`); this module wires them to the database and
//! the per-installation data directory. Only RPG Maker MV/MZ projects are
//! snapshotted: Wolf and VX Ace exports keep reading the game folder.

use std::path::{Path, PathBuf};

use sqlx::SqlitePool;

use crate::engines::filter::{classify_file_type, FileClass};
use crate::engines::mv_mz::originals::{reconstruct_json, CoreSegment, SnapshotStore};

/// Snapshot directory of one project.
pub fn snapshot_root(data_dir: &Path, project_id: &str) -> PathBuf {
    data_dir.join("originals").join(project_id)
}

/// Path of `file_path` relative to the game folder with `/` separators — the
/// key used both inside snapshots and as the zip entry name.
pub fn game_relative_path(game_path: &str, file_path: &str, fallback: &str) -> String {
    Path::new(file_path)
        .strip_prefix(Path::new(game_path))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| fallback.to_string())
}

/// Remove a project's snapshot (best effort: a leftover directory is harmless,
/// a failure here must never block deleting or re-creating the project).
pub fn remove_snapshot(data_dir: &Path, project_id: &str) {
    let root = snapshot_root(data_dir, project_id);
    if root.exists() {
        if let Err(error) = std::fs::remove_dir_all(&root) {
            log::warn!("could not remove snapshot {}: {error}", root.display());
        }
    }
}

/// Make sure every MV/MZ JSON source file of `project_id` has a pristine
/// snapshot, capturing (and if needed reconstructing) missing ones from disk.
/// Idempotent: files already snapshotted are never re-read from the game.
pub async fn ensure_mv_mz_snapshot(
    pool: &SqlitePool,
    data_dir: &Path,
    project_id: &str,
) -> Result<SnapshotStore, String> {
    let (game_path, source_language): (String, String) = sqlx::query_as(
        "SELECT game_path, COALESCE(source_language, 'ja') FROM projects WHERE id = ?",
    )
    .bind(project_id)
    .fetch_one(pool)
    .await
    .map_err(|error| error.to_string())?;

    let files: Vec<(String, String, String, String)> = sqlx::query_as(
        "SELECT id, file_name, file_path, file_type FROM source_files WHERE project_id = ?",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
    .map_err(|error| error.to_string())?;

    let root = snapshot_root(data_dir, project_id);
    let mut store = tokio::task::spawn_blocking(move || SnapshotStore::open(root))
        .await
        .map_err(|error| error.to_string())?
        .map_err(|error| error.to_string())?;

    let mut changed = false;
    for (file_id, file_name, file_path, file_type) in files {
        if !matches!(classify_file_type(&file_type), FileClass::Json) {
            continue;
        }
        let rel = game_relative_path(&game_path, &file_path, &file_name);
        if store.contains(&rel) {
            continue;
        }
        let segments: Vec<(String, String, String)> = sqlx::query_as(
            "SELECT json_key, source_text, target_text FROM segments WHERE source_file_id = ?",
        )
        .bind(&file_id)
        .fetch_all(pool)
        .await
        .map_err(|error| error.to_string())?;
        let segments: Vec<CoreSegment> = segments
            .into_iter()
            .map(|(key, source, target)| CoreSegment {
                key,
                source,
                target,
            })
            .collect();

        let language = source_language.clone();
        let label = rel.clone();
        let (store_back, outcome) = tokio::task::spawn_blocking(move || {
            let result = std::fs::read(&file_path)
                .map_err(|error| format!("read {label}: {error}"))
                .and_then(|disk| {
                    reconstruct_json(&label, &disk, &segments, &language)
                        .map_err(|error| error.to_string())
                })
                .and_then(|rec| {
                    if !rec.drift.is_empty() {
                        log::warn!(
                            "{label}: {} extracted pointer(s) differ from the recorded source; \
                             the game file changed since extraction",
                            rec.drift.len()
                        );
                    }
                    store
                        .put(&label, &rec.bytes, rec.provenance, rec.drift)
                        .map_err(|error| error.to_string())
                });
            (store, result)
        })
        .await
        .map_err(|error| error.to_string())?;
        store = store_back;
        outcome?;
        changed = true;
    }

    if changed {
        store.save().map_err(|error| error.to_string())?;
    }
    Ok(store)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engines::mv_mz::originals::Provenance;

    async fn seeded(game: &Path, map_on_disk: &str) -> (SqlitePool, tempfile::NamedTempFile) {
        let db = tempfile::NamedTempFile::new().unwrap();
        let pool = crate::db::pool::init(db.path().to_str().unwrap())
            .await
            .unwrap();
        let data = game.join("data");
        std::fs::create_dir_all(&data).unwrap();
        std::fs::write(data.join("Map001.json"), map_on_disk).unwrap();
        sqlx::query(
            "INSERT INTO projects (id, name, engine, game_path) VALUES ('p1', 'T', 'mv_mz', ?)",
        )
        .bind(game.to_string_lossy().to_string())
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO source_files (id, project_id, file_name, file_path, file_type) \
             VALUES ('f1', 'p1', 'Map001.json', ?, 'map')",
        )
        .bind(data.join("Map001.json").to_string_lossy().to_string())
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO segments (id, source_file_id, json_key, source_text, target_text, status) \
             VALUES ('s1', 'f1', '/a', '勇者', 'Hero', 'translated')",
        )
        .execute(&pool)
        .await
        .unwrap();
        (pool, db)
    }

    #[tokio::test]
    async fn snapshot_reconstructs_an_already_exported_folder_once() {
        let temp = tempfile::tempdir().unwrap();
        let game = temp.path().join("game");
        // The user unzipped a previous export over the game: disk holds the target.
        let (pool, _db) = seeded(&game, r#"{"a":"\\FS[22]Hero","b":"不変"}"#).await;
        let data_dir = temp.path().join("app");

        let store = ensure_mv_mz_snapshot(&pool, &data_dir, "p1").await.unwrap();
        assert_eq!(
            store.entry("data/Map001.json").unwrap().provenance,
            Provenance::Reconstructed
        );
        assert_eq!(
            store.read("data/Map001.json").unwrap(),
            r#"{"a":"勇者","b":"不変"}"#.as_bytes()
        );

        // Idempotent: later disk changes are never re-captured.
        std::fs::write(game.join("data").join("Map001.json"), r#"{"a":"garbage"}"#).unwrap();
        let again = ensure_mv_mz_snapshot(&pool, &data_dir, "p1").await.unwrap();
        assert_eq!(
            again.read("data/Map001.json").unwrap(),
            r#"{"a":"勇者","b":"不変"}"#.as_bytes()
        );

        remove_snapshot(&data_dir, "p1");
        assert!(!snapshot_root(&data_dir, "p1").exists());
    }

    #[test]
    fn relative_paths_use_forward_slashes_and_fall_back_to_the_file_name() {
        let game = Path::new("C:/games/g");
        let file = game.join("www").join("data").join("Map001.json");
        assert_eq!(
            game_relative_path(
                &game.to_string_lossy(),
                &file.to_string_lossy(),
                "Map001.json"
            ),
            "www/data/Map001.json"
        );
        assert_eq!(
            game_relative_path("/elsewhere", "/other/Map001.json", "Map001.json"),
            "Map001.json"
        );
    }
}
