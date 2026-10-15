# Journal — 2026-09-21 — Installation BMAD

**Outil :** Codex.
**Branche :** `codex/shared-project-continuity` ; HEAD de départ `7c1b6b9`.
**Demande :** installer BMAD pour Claude et Codex dans le projet existant.
**Périmètre :** installation locale et documentation ; aucun code applicatif.
**Statut :** installation réussie ; contrôles locaux réussis.

## Réalisation

- Version stable 6.12.0 vérifiée sur le registre npm officiel avant téléchargement.
- Node.js 24.12.0, pnpm 11.6.0 et uv 0.12.10 disponibles.
- Exécution de l'installateur officiel avec BMM et `--tools claude-code,codex`.
- Core + BMM 6.12.0 installés ; 29 skills dans chacun des dossiers propres aux outils.
- Projet Hoshi2Stars, communication et documents en français ; documentation
  existante `docs`, sorties `_bmad-output` ; aucun module additionnel ni shim obsolète.
- Protocole `AGENTS.md` / `CLAUDE.md` conservé, point de reprise actualisé.
- Exclusions Git affinées pour partager uniquement les nouveaux skills BMAD
  sous `.agents/skills` ; autres skills locaux toujours ignorés.
- Réponses personnelles de l'installateur exclues de Git ; langue commune
  définie dans `_bmad/custom/config.toml`, conservée aux mises à jour.
- Commandes reproductibles et utilisation documentées dans `docs/bmad.md`.

## Vérifications

- L'installateur termine avec succès pour les deux intégrations.
- La commande BMAD `status` confirme Core et BMM 6.12.0.
- Le résolveur de configuration s'exécute via `uv` et restitue le nom du projet,
  la langue française et les chemins attendus.
- Manifeste vérifié : 29 skills par outil ; 214 fichiers par intégration,
  identiques par empreintes SHA-256 ; tous les nouveaux skills partageables par Git.
- Préservation vérifiée sur l'inventaire de 613 fichiers avant intervention :
  seuls `AGENTS.md`, `tasks/handoff.md`, `tasks/todo.md` et `.gitignore` ont changé.
  Les 609 autres fichiers sont inchangés, dont `CLAUDE.md`, les leçons, les anciens
  skills, les dépendances et le code applicatif.
- Rendu local de `bmad-build` réussi via `uv run --no-cache` et
  `_bmad/scripts/render_skill.py`, depuis `.agents/skills/bmad-build` puis
  `.claude/skills/bmad-build`. Les deux produisent le même rendu
  `_bmad/render/bmad-build/hoshi2star-6e1f4d5ced7b/821acbd12592bd3a1c65/workflow.md`.
  Test de préparation uniquement : les instructions du workflow n'ont pas été exécutées.
- Exclusions Git vérifiées : configuration personnelle et rendus ignorés,
  anciens skills locaux toujours ignorés, skills BMAD désormais partageables.
- Import Claude `@AGENTS.md` et suivi partagé conservés ; `git diff --check` réussi.

## Limites et prochaine action

Les fichiers installés et les scripts locaux sont vérifiés, mais aucune nouvelle
session modèle Claude/Codex n'est lancée ici et aucun workflow de développement
applicatif n'est exécuté. Les tests de l'application ne sont pas relancés.

Ouvrir une nouvelle session dans le même dépôt et demander `bmad-help` avec la
reprise décrite dans `tasks/handoff.md`. Aucun commit, push ni déploiement effectués.
