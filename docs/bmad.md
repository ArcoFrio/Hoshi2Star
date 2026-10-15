# BMAD pour Codex et Claude Code

Installation locale du 2026-09-21 : **BMAD 6.12.0, Core + BMM**.
Les deux outils utilisent le même dépôt et la même configuration.

## Commencer ou reprendre

Ouvrir `/home/blackat/Bureau/Hoshi2Star` dans une nouvelle session Codex ou Claude
Code pour charger les skills installés. Demande utilisable dans les deux outils :

> Lis AGENTS.md, tasks/handoff.md, tasks/lessons.md et le journal lié, puis utilise
> bmad-help pour indiquer où nous en sommes et la prochaine action. Vérifie
> l'état Git et ne modifie encore aucun fichier.

Les fichiers de suivi doivent être actualisés après chaque étape significative.
Les conversations privées ne sont pas synchronisées ; voir `AGENTS.md`.

## Emplacements

| Élément | Emplacement |
| --- | --- |
| 29 skills Codex | `.agents/skills/bmad-*/` |
| 29 skills Claude | `.claude/skills/bmad-*/` |
| Configuration et scripts communs | `_bmad/` |
| Personnalisations partagées, conservées aux réinstallations | `_bmad/custom/config.toml` |
| Plans et spécifications BMAD | `_bmad-output/planning-artifacts/` |
| Stories, suivi et résultats d'implémentation BMAD | `_bmad-output/implementation-artifacts/` |
| Documentation existante | `docs/` |
| Point de reprise et leçons des deux outils | `tasks/handoff.md`, `tasks/lessons.md` |

Les réponses et les documents sont configurés en français. Les sources des
skills officiels restent dans leur langue d'origine. Les noms de rôles BMAD
(analyste, architecte, développeur, etc.) sont distincts des outils Codex et Claude.

## Vérifier ou réinstaller la même version

Depuis la racine du projet :

```bash
pnpm dlx bmad-method@6.12.0 status
```

Commande d'installation utilisée :

```bash
pnpm dlx bmad-method@6.12.0 install --directory . --modules bmm --tools claude-code,codex --communication-language French --document-output-language French --output-folder _bmad-output --set core.project_name=Hoshi2Star --set bmm.project_knowledge=docs --no-shims --yes
```

Ne pas modifier les copies générées des skills pour ajouter une règle de projet :
utiliser `AGENTS.md` pour la continuité et `_bmad/custom/` pour les personnalisations
BMAD prises en charge. Une mise à jour doit conserver ce protocole.

Sur une autre machine ou un autre clone, transférer via Git le code, les skills,
la configuration commune, le suivi et les sorties utiles. Les réponses personnelles
`_bmad/config.user.toml` et les rendus temporaires ne sont pas suivis. Aucun commit
de cette installation n'est effectué automatiquement.

Référence : [installation officielle BMAD](https://docs.bmad-method.org/start/install-bmad/).
