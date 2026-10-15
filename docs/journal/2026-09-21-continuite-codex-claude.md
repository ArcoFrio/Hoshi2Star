# Journal — 2026-09-21 — Continuité Codex / Claude

**Outil :** Codex.
**Branche :** `codex/shared-project-continuity`, issue de `main`.
**Commit de départ :** `7c1b6b9` ; modifications locales préexistantes conservées.
**Périmètre :** instructions et suivi partagé, sans code applicatif ni installation BMAD.
**Statut :** fichiers configurés ; contrôles documentaires réussis ;
reprise réelle dans une nouvelle session de l'autre outil à vérifier.

## Actions et décisions

- Création de `AGENTS.md`, lu par Codex et importé depuis `CLAUDE.md`.
- Création de `tasks/handoff.md`, point de reprise unique des deux outils.
- Adaptation des règles de journal de `CONTEXT.md` et de
  `docs/journal/TEMPLATE.md` pour enregistrer outil, corrections, preuves et limites.
- Conservation de l'historique existant dans `tasks/todo.md`, `tasks/lessons.md`,
  `docs/journal/`, les plans et les ADRs.
- Ajout d'une leçon documentée sur la divergence entre roadmap et code.
- Actualisation du point de reprise après chaque étape significative requise,
  afin de réduire la perte de contexte en cas d'interruption.
- BMAD pourra référencer ce suivi ; sa présence n'est pas nécessaire au protocole.

## Constat corrigé dans les consignes

La journalisation était formulée pour Claude seulement, sans fichier racine
`AGENTS.md` ni point de reprise unique. Les deux outils sont maintenant orientés
vers les mêmes fichiers. Cela ne synchronise pas leurs conversations privées.

Une seconde observation empêche une reprise erronée : la case provider de
`ROADMAP.md` est encore ouverte, mais `OpenAiCompatibleProvider` est présent dans
`src-tauri/src/llm/provider.rs` et appelé dans
`src-tauri/src/commands/translate.rs`. La roadmap complète n'a pas été réécrite,
et aucun test fournisseur n'a été exécuté. La règle de reprise demande de
réconcilier l'état déclaré avec le code avant de réimplémenter.

## Vérifications et limites

- Inspection Git et documentaire réalisée ; aucun code applicatif modifié.
- Contrôle de préservation par empreintes SHA-256 : parmi les 358 fichiers
  inventoriés avant intervention, seuls les quatre documents prévus ont changé
  (`CLAUDE.md`, `CONTEXT.md`, `tasks/lessons.md`, `docs/journal/TEMPLATE.md`).
  Les 354 autres fichiers sont inchangés.
- Vérification du contenu antérieur de `CLAUDE.md` et de `tasks/lessons.md` :
  intégralement conservé ; `CONTEXT.md` ne change ici que dans son bloc de
  continuité. Une nouvelle ligne finale a été ajoutée à des fichiers qui en manquaient.
- Les trois nouveaux fichiers (`AGENTS.md`, `tasks/handoff.md`, ce journal)
  existent et ne sont pas ignorés par Git ; aucun staging ou commit effectué.
- Import `@AGENTS.md` unique, sans import circulaire ; références locales
  présentes ; instructions communes inférieures à 10 Ko.
- `git diff --check` réussi ; nouveaux fichiers sans espaces de fin de ligne.
- Tests applicatifs non exécutés ; nouvelle session Claude non lancée.
- Aucun commit, push, déploiement ou installation BMAD.

## Prochaine session

Suivre le contrôle de reprise en lecture seule décrit dans `tasks/handoff.md`.
Puis poursuivre la demande utilisateur et mettre à jour le même suivi, quel que
soit l'outil. Les recommandations de l'audit MV/MZ ne sont pas une autorisation
de commencer des corrections applicatives.
