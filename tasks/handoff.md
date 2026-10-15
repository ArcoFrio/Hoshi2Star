# Point de reprise — Hoshi2Star

**Mis à jour :** 2026-10-15 — Claude.
**Objectif utilisateur :** intégrer les PR externes #5/#6 d'ArcoFrio en sécurité, après
avoir consolidé `main`.
**Statut :** revue faite, base sauvegardée, nom affiché fixé à **Hoshi2Star**, travail
local commité sur `codex/shared-project-continuity`. Aucune PR fusionnée.

## Dernier point atteint

- Journal : `docs/journal/2026-10-15-revue-pr-arcofrio.md` (gates, constats, décisions).
- Session précédente (protocole Codex/Claude, BMAD 6.12.0) :
  `docs/journal/2026-09-21-continuite-codex-claude.md`, `docs/journal/2026-09-21-installation-bmad.md`.
- Sauvegarde base : `~/.local/share/com.blackat.hoshi2star/backups/hoshi2star-before-pr-merge-2026-10-15-1234.db`.
- La base locale a appliqué les migrations 1–9 ; la 9 vient de `codex/hoshiyomi-redesign`.

## Dossier et état Git à retrouver

- Dossier : `/home/blackat/Bureau/Hoshi2Star`, branche `codex/shared-project-continuity`.
- `origin/main` @ `7737d9f`. `codex/hoshiyomi-redesign` @ `04dd416` (2 commits : refonte UI
  « Hoshiyomi » + terminologie simplifiée avec migration 0009), non fusionnée.
- PR #5 (`ArcoFrio:feat/mv-mz-plugin-text`) et #6 (`feat/mv-mz-text-fitting`, empilée sur #5) ouvertes.
- Non suivis volontairement : `research_notes/`, `reports/` (rapports de recherche, emplacement
  à décider), `.claude/skills/find-skills`, `.claude/skills/make-interfaces-feel-better`.

## Contexte utile, sans refaire le travail

- Nom affiché : Hoshi2Star. `productName` = `hoshi2star`, identifiant `com.blackat.hoshi2star`
  inchangés. Ne pas réintroduire « Hoshi2Stars » ni « Hoshiyomi ».
- Ne pas fusionner #5 avant `codex/hoshiyomi-redesign` : sinon `VersionMissing(9)` au démarrage.
- #5 contient une faille bloquante (`core/originals.rs`, `project_id` non validé avant
  `remove_dir_all`). Détails et liste des correctifs demandés dans le journal du 2026-10-15.
- `OpenAiCompatibleProvider` existe déjà (voir `tasks/lessons.md`).

## Vérifications

- Gates complets passés sur les têtes de #5 et #6 (Linux). Tests sur vrais jeux, tests
  Windows et test manuel dans l'app : non exécutés.
- Commit local : contrôlé par `git diff --cached --stat` avant commit ; aucun code Rust
  fonctionnel modifié (chaînes et docs seulement).

## Prochaine action exacte

1. Créer une PR qui fusionne `codex/hoshiyomi-redesign` dans `main` en remplaçant
   « Hoshiyomi » par « Hoshi2Star » ; lancer le gate complet.
2. L'utilisateur envoie la réponse à ArcoFrio (correctifs + rebase sur le nouveau `main`
   + question ouverte sur les modèles locaux 8/12/16 Go).
3. À réception : nouvelle revue + gate, test sur 2–3 jeux MV/MZ, merge #5 puis #6,
   puis PR de petits correctifs.
