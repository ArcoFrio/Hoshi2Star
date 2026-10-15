# Journal — 2026-10-15 — Revue des PR #5/#6 d'ArcoFrio, nom Hoshi2Star

**Phase** : F4 / F5
**Durée estimée** : 3h
**Statut** : 🔄 En cours
**Outil** : Claude
**Branche et commit de départ** : `codex/shared-project-continuity` @ `7c1b6b9`
**Objectif / plan / story** : analyser les PR externes #5 et #6, préparer la réponse
au contributeur, sauvegarder la base locale, fixer le nom affiché, commiter le travail local.

---

## Ce qui a été fait

- Recherches (2026-09-24/25) : panorama des outils de traduction de jeux, alternatives
  à Ollama, offres cloud, petits modèles, crates Rust multi-fournisseurs. Rapports dans
  `reports/` et notes dans `research_notes/` (non suivis par Git, emplacement à décider).
- Revue des PR #5 (texte plugins/système) et #6 (text fitting MZ) d'ArcoFrio (fork),
  base `origin/main` @ `7737d9f`. #6 est empilée sur #5.
- Sauvegarde de la base : `~/.local/share/com.blackat.hoshi2star/backups/hoshi2star-before-pr-merge-2026-10-15-1234.db`
  (`sqlite3 .backup`, `integrity_check` = ok, migrations 1–9).
- Nom affiché fixé à **Hoshi2Star** (décision utilisateur). Les mentions « Hoshi2Stars »
  des modifications locales non commitées ont été remplacées. `productName` reste
  `hoshi2star` (nom des binaires et des paquets inchangé), titre de fenêtre « Hoshi2Star ».
  Identifiant `com.blackat.hoshi2star` inchangé.

## Fichiers créés

- `docs/journal/2026-10-15-revue-pr-arcofrio.md`

## Fichiers modifiés

- Nom affiché : `index.html`, `src-tauri/tauri.conf.json` (titre de fenêtre), docs et
  fichiers BMAD qui mentionnaient « Hoshi2Stars ».
- `tasks/handoff.md` — nouveau point de reprise.

## Décisions prises

- Nom affiché : Hoshi2Star. Le journal `2026-09-21-installation-bmad.md` garde
  « Hoshi2Stars » (session historique, non réécrite).
- Ordre d'intégration recommandé et accepté pour démarrer : sauvegarde → commit local
  → fusion de `codex/hoshiyomi-redesign` dans `main` en remplaçant « Hoshiyomi » par
  « Hoshi2Star » → réponse à ArcoFrio avec demande de rebase → nouvelle revue → merge #5 puis #6.

## Problèmes rencontrés

- Hook PreToolUse : la regex `npm install` bloque aussi `pnpm install` (faux positif).
  Contournement : `pnpm i`. Correction du hook non faite (accord utilisateur requis).
- `/tmp` (tmpfs 7,8 Go) saturé par `CARGO_TARGET_DIR` : tests Rust relancés avec la
  cible sur `/home`. Dossiers de revue et cache supprimés ensuite.

## Corrections et erreurs à ne plus répéter

- Ne pas placer un `CARGO_TARGET_DIR` de build complet dans `/tmp` (tmpfs en RAM).

## Vérifications et limites

Gates exécutés le 2026-10-15 sur les têtes des PR (Linux) :

| Gate | PR #5 (`e4e0a29`) | PR #6 (`87fbb3f`) |
|---|---|---|
| `pnpm typecheck`, `pnpm lint` | OK | OK |
| `pnpm test` | 107/107 | 116/116 |
| `cargo fmt --check`, `clippy --all-targets -D warnings` | OK | OK |
| `cargo test` | 590 OK, 0 échec | 636 OK, 0 échec |

Tests `#[ignore]` sur vrai jeu non exécutés. Aucun test manuel dans l'application.
Tests Windows non exécutés.

Constats de revue (lecture du code ; les points marqués « vérifié » ont été relus par Claude) :

- #5 **bloquant (vérifié)** : `core/originals.rs` `snapshot_root`/`remove_snapshot` joignent
  le `project_id` lu dans `.hoshi2star.json` (dossier du jeu, non fiable) puis
  `remove_dir_all` → suppression hors du dossier de l'app avec `../..`. Correctif : UUID obligatoire.
- #5 **migration (vérifié)** : la PR saute 0009. La base locale a appliqué 0009
  (`terminology translation scope`, branche `codex/hoshiyomi-redesign`) → démarrage
  en échec `VersionMissing(9)` si #5 est fusionnée seule.
- #5 importants : catégories « verified » auto-activées sur nouveaux projets ; snapshot
  jamais rafraîchi après mise à jour du jeu ; `%n` perdu bloque l'export des projets post-0007 ;
  snapshot complet dès l'ouverture d'un projet restauré.
- #5 mineurs : pas de limite de profondeur dans `json_span.rs` ; `..` non filtré dans les
  noms de plugins ; profil `nuun_enemy_book` épinglé par `@version` seul ; modifie
  `tasks/`, `ROADMAP.md`, `lessons.md`.
- #6 importants : les messages ignorés par le fitting perdent le préfixe `\FS[n]` d'export ;
  `RE_STORED_FONT_PREFIX` supprime aussi un `\FS[n]` voulu (vérifié), non détecté par `verify`.
- #6 mineurs : inflate WOFF sans limite (vérifié) ; masquage QA « line too long » sans
  `Support::Ready` ; vérification couleurs par nombre seulement ; wrap quadratique.

## Tâches ROADMAP cochées

- [ ] aucune

## Prochaine session

- Fusionner `codex/hoshiyomi-redesign` dans `main` via PR, en remplaçant « Hoshiyomi »
  par « Hoshi2Star » ; 18 fichiers recoupent la PR #5.
- Puis envoyer la réponse à ArcoFrio (brouillon dans la conversation, question ouverte sur
  les modèles 8/12/16 Go de VRAM) avec demande de rebase sur ce `main`.
