//! Optional text coverage for RPG Maker MV/MZ: player-visible text that core
//! extraction does not cover (System type lists, map display names, and — in
//! later modules — plugin parameters, plugin-command arguments and note tags).
//!
//! Each candidate group is a [`TextCategory`] with an explicit safety state:
//!
//! - **Verified** — display-only and referenced nowhere as an identifier;
//!   enabled by default.
//! - **Linked** — referenced as an identifier, but every reference is covered
//!   by a known rewrite rule; opt-in, with consequences shown first.
//! - **Blocked** — referenced somewhere Hoshi2Star cannot update safely; never
//!   extracted, the exact locations are reported.
//!
//! The scan is pure: callers pass the pristine game files and receive the
//! report. Nothing here touches the database or the game folder.

pub mod sites;

use serde::Serialize;
use serde_json::{json, Value};

use self::sites::{excerpt, token_occurs, Site, SiteKind};
use crate::engines::filter;
use crate::engines::mv_mz::extractor::SegmentKind;
use crate::engines::mv_mz::plugins_js::PluginsJs;
use crate::llm::tokenizer::Engine as TokEngine;

/// Bump when classification rules change (part of the scan digest).
pub const SCANNER_VERSION: &str = "mv_mz-coverage-v1";

/// Candidate strings longer than this, or spanning lines, are prose and are
/// never used as identifiers; they are not reference-checked.
const MAX_IDENTIFIER_CHARS: usize = 40;
/// References kept per category in the report (counts stay exact).
const MAX_REFERENCES_SHOWN: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CategoryState {
    Verified,
    Linked,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CategoryGroup {
    System,
    Map,
    Plugin,
    PluginCommand,
    NoteTag,
}

/// A rule the translation of a category must satisfy to be injected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "rule")]
pub enum Constraint {
    /// Every `%n` of the source must be kept.
    KeepFormatSpecifiers,
    /// The value is drawn with `drawText`: escape codes (`\C[n]`, `\FS[n]`…)
    /// would be shown literally.
    NoEscapeCodes,
}

/// A string that enabling the category turns into a segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    /// Data/JS file name as stored in `source_files.file_name`.
    pub file_name: String,
    /// Game-relative path (snapshot key / zip entry).
    pub file_rel: String,
    pub key: String,
    pub source: String,
    pub kind: SegmentKind,
    pub context_json: String,
}

/// One place where a candidate string is used as an identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    pub text: String,
    pub site: SiteKind,
    pub location: String,
    pub excerpt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextCategory {
    pub key: String,
    pub title: String,
    pub group: CategoryGroup,
    pub state: CategoryState,
    pub default_enabled: bool,
    /// Where the player sees this text.
    pub visible_in: String,
    /// What enabling it changes, in plain language.
    pub consequences: Vec<String>,
    pub constraints: Vec<Constraint>,
    pub candidates: Vec<Candidate>,
    /// References that make the category blocked (or linked), first
    /// [`MAX_REFERENCES_SHOWN`] only.
    pub references: Vec<Reference>,
    pub reference_count: usize,
    pub blocked_reason: Option<String>,
}

/// An existing (core) translation whose source is also used as an identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreWarning {
    pub segment_kind: String,
    pub source: String,
    pub references: Vec<Reference>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CoverageReport {
    pub categories: Vec<TextCategory>,
    pub core_warnings: Vec<CoreWarning>,
    /// Set when `plugins.js` exists but could not be read safely: plugin
    /// parameters are then neither scanned for references nor offered.
    pub plugins_unreadable: Option<String>,
}

/// One core segment considered for identifier warnings.
#[derive(Debug, Clone)]
pub struct CoreSource {
    pub segment_kind: String,
    pub source: String,
}

/// Everything the scan needs, already loaded from the pristine snapshot.
pub struct CoverageInput<'a> {
    /// `(file_name, game-relative path, parsed JSON)` for every data file.
    pub data_files: &'a [(String, String, Value)],
    /// `plugins.js` original text, if the game has one.
    pub plugins_js: Option<&'a str>,
    /// `(plugin name, source text)` of enabled plugins found on disk.
    pub plugin_sources: &'a [(String, String)],
    /// Canonical core segments to check for identifier use.
    pub core_sources: &'a [CoreSource],
}

/// Scan the game and classify every optional text category.
pub fn scan(input: &CoverageInput<'_>) -> CoverageReport {
    let mut sites = Vec::new();
    let named: Vec<(String, Value)> = input
        .data_files
        .iter()
        .map(|(name, _, json)| (name.clone(), json.clone()))
        .collect();
    sites::collect_data_sites(&named, &mut sites);

    let mut report = CoverageReport::default();
    if let Some(text) = input.plugins_js {
        match PluginsJs::parse(text) {
            Ok(plugins) => sites::collect_plugin_param_sites(&plugins, &mut sites),
            Err(error) => report.plugins_unreadable = Some(error.to_string()),
        }
    }
    for (plugin, source) in input.plugin_sources {
        sites::collect_plugin_source_sites(plugin, source, &mut sites);
    }

    let mut categories = system_type_categories(input.data_files);
    categories.extend(map_display_name_category(input.data_files));
    for category in &mut categories {
        classify(category, &sites);
    }
    categories.retain(|category| !category.candidates.is_empty());
    report.categories = categories;
    report.core_warnings = core_warnings(input.core_sources, &sites);
    report
}

/// Apply the reference check to `category`.
fn classify(category: &mut TextCategory, sites: &[Site]) {
    let references = find_references(category.candidates.iter().map(|c| c.source.as_str()), sites);
    category.reference_count = references.len();
    if references.is_empty() {
        category.state = CategoryState::Verified;
        category.default_enabled = true;
        category.blocked_reason = None;
    } else {
        category.state = CategoryState::Blocked;
        category.default_enabled = false;
        category.blocked_reason = Some(format!(
            "{} of these texts {} used as an identifier in {} place(s) Hoshi2Star cannot \
             update safely (listed below). Translating them would break those lookups.",
            distinct_texts(&references),
            if distinct_texts(&references) == 1 {
                "is"
            } else {
                "are"
            },
            references.len()
        ));
    }
    category.references = references.into_iter().take(MAX_REFERENCES_SHOWN).collect();
}

fn distinct_texts(references: &[Reference]) -> usize {
    references
        .iter()
        .map(|reference| reference.text.as_str())
        .collect::<std::collections::HashSet<_>>()
        .len()
}

/// Token-bounded occurrences of `texts` in `sites`.
fn find_references<'a>(texts: impl Iterator<Item = &'a str>, sites: &[Site]) -> Vec<Reference> {
    let mut unique: Vec<&str> = texts
        .map(str::trim)
        .filter(|text| {
            !text.is_empty() && !text.contains('\n') && text.chars().count() <= MAX_IDENTIFIER_CHARS
        })
        .collect();
    unique.sort_unstable();
    unique.dedup();
    let mut references = Vec::new();
    for site in sites {
        for text in &unique {
            if let Some(at) = token_occurs(text, &site.text) {
                references.push(Reference {
                    text: (*text).to_string(),
                    site: site.kind,
                    location: site.location.clone(),
                    excerpt: excerpt(&site.text, at, text.len()),
                });
            }
        }
    }
    references
}

fn core_warnings(core_sources: &[CoreSource], sites: &[Site]) -> Vec<CoreWarning> {
    let mut warnings: Vec<CoreWarning> = Vec::new();
    for source in core_sources {
        if warnings
            .iter()
            .any(|w| w.source == source.source && w.segment_kind == source.segment_kind)
        {
            continue;
        }
        let references = find_references(std::iter::once(source.source.as_str()), sites);
        if !references.is_empty() {
            warnings.push(CoreWarning {
                segment_kind: source.segment_kind.clone(),
                source: source.source.clone(),
                references: references.into_iter().take(MAX_REFERENCES_SHOWN).collect(),
            });
        }
    }
    warnings
}

/// Shared `context_json` for category segments.
pub fn category_context(category: &str, extra: Value) -> String {
    let mut context = json!({
        "schemaVersion": 1,
        "engine": "rpg_maker_mv_mz",
        "category": category,
    });
    if let (Value::Object(base), Value::Object(more)) = (&mut context, extra) {
        base.extend(more);
    }
    context.to_string()
}

struct SystemList {
    field: &'static str,
    key: &'static str,
    title: &'static str,
    visible_in: &'static str,
}

const SYSTEM_LISTS: &[SystemList] = &[
    SystemList {
        field: "skillTypes",
        key: "system.skill_types",
        title: "Skill type names",
        visible_in: "Skill menu tabs and the battle Skill command",
    },
    SystemList {
        field: "equipTypes",
        key: "system.equip_types",
        title: "Equipment slot names",
        visible_in: "Equip screen slot labels",
    },
    SystemList {
        field: "elements",
        key: "system.elements",
        title: "Element names",
        visible_in:
            "Only where a plugin shows them (e.g. bestiary weaknesses); not in default menus",
    },
    SystemList {
        field: "weaponTypes",
        key: "system.weapon_types",
        title: "Weapon type names",
        visible_in: "Only where a plugin shows them; not in default menus",
    },
    SystemList {
        field: "armorTypes",
        key: "system.armor_types",
        title: "Armor type names",
        visible_in: "Only where a plugin shows them; not in default menus",
    },
];

fn system_type_categories(files: &[(String, String, Value)]) -> Vec<TextCategory> {
    let Some((file_name, file_rel, system)) =
        files.iter().find(|(name, _, _)| name == "System.json")
    else {
        return Vec::new();
    };
    SYSTEM_LISTS
        .iter()
        .map(|list| {
            let candidates = system
                .get(list.field)
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .enumerate()
                        .filter_map(|(index, value)| {
                            let text = value.as_str()?;
                            filter::needs_translation(text, TokEngine::MvMz).then(|| Candidate {
                                file_name: file_name.clone(),
                                file_rel: file_rel.clone(),
                                key: format!("/{}/{index}", list.field),
                                source: text.to_string(),
                                kind: SegmentKind::SystemType,
                                context_json: category_context(
                                    list.key,
                                    json!({ "field": list.field, "index": index }),
                                ),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            TextCategory {
                key: list.key.to_string(),
                title: list.title.to_string(),
                group: CategoryGroup::System,
                state: CategoryState::Verified,
                default_enabled: true,
                visible_in: list.visible_in.to_string(),
                consequences: vec![
                    format!(
                        "Translates the `{}` list of System.json; the game reads these names only by \
                         their index, so translating them is safe unless a plugin looks them up by name \
                         (checked below).",
                        list.field
                    ),
                    "Names are drawn without escape codes: keep them plain text.".to_string(),
                ],
                constraints: vec![Constraint::NoEscapeCodes],
                candidates,
                references: Vec::new(),
                reference_count: 0,
                blocked_reason: None,
            }
        })
        .collect()
}

fn map_display_name_category(files: &[(String, String, Value)]) -> Option<TextCategory> {
    let candidates: Vec<Candidate> = files
        .iter()
        .filter(|(name, _, _)| filter::is_map_file(name, ".json"))
        .filter_map(|(file_name, file_rel, map)| {
            let text = map.get("displayName").and_then(Value::as_str)?;
            filter::needs_translation(text, TokEngine::MvMz).then(|| Candidate {
                file_name: file_name.clone(),
                file_rel: file_rel.clone(),
                key: "/displayName".to_string(),
                source: text.to_string(),
                kind: SegmentKind::MapDisplayName,
                context_json: category_context(
                    "map.display_name",
                    json!({ "field": "displayName" }),
                ),
            })
        })
        .collect();
    Some(TextCategory {
        key: "map.display_name".to_string(),
        title: "Map display names".to_string(),
        group: CategoryGroup::Map,
        state: CategoryState::Verified,
        default_enabled: true,
        visible_in: "The map name popup shown when entering a map".to_string(),
        consequences: vec![
            "Translates each map's `displayName`. The editor-only map list names are already part \
             of the core extraction."
                .to_string(),
            "The popup draws plain text: the font-size prefix is never added to these names."
                .to_string(),
        ],
        constraints: vec![Constraint::NoEscapeCodes],
        candidates,
        references: Vec::new(),
        reference_count: 0,
        blocked_reason: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn system(lists: Value) -> (String, String, Value) {
        ("System.json".into(), "data/System.json".into(), lists)
    }

    fn category<'a>(report: &'a CoverageReport, key: &str) -> &'a TextCategory {
        report
            .categories
            .iter()
            .find(|c| c.key == key)
            .unwrap_or_else(|| panic!("missing {key}"))
    }

    #[test]
    fn unreferenced_type_lists_are_verified_and_referenced_ones_blocked() {
        let files = vec![
            system(json!({
                "skillTypes": ["", "魔術", "鞭"],
                "armorTypes": ["", "軽装防具"],
                "equipTypes": ["", "武器"],
                "elements": ["", "123"]
            })),
            (
                "Weapons.json".into(),
                "data/Weapons.json".into(),
                json!([null, {"note": "<コマンド:スキル/鞭,48,鞭攻撃,76>"}]),
            ),
        ];
        let report = scan(&CoverageInput {
            data_files: &files,
            plugins_js: None,
            plugin_sources: &[],
            core_sources: &[],
        });

        let skills = category(&report, "system.skill_types");
        assert_eq!(skills.state, CategoryState::Blocked);
        assert!(!skills.default_enabled);
        assert_eq!(skills.reference_count, 1);
        assert_eq!(skills.references[0].text, "鞭");
        assert_eq!(skills.references[0].site, SiteKind::Note);
        assert_eq!(skills.candidates.len(), 2);

        let armor = category(&report, "system.armor_types");
        assert_eq!(armor.state, CategoryState::Verified);
        assert!(armor.default_enabled);
        assert_eq!(armor.candidates[0].key, "/armorTypes/1");
        assert_eq!(armor.candidates[0].kind, SegmentKind::SystemType);
        assert!(armor.candidates[0]
            .context_json
            .contains(r#""category":"system.armor_types""#));

        // Pure numbers are not offered; empty categories disappear.
        assert!(report.categories.iter().all(|c| c.key != "system.elements"));
        assert!(report
            .categories
            .iter()
            .all(|c| c.key != "map.display_name"));
    }

    #[test]
    fn plugin_params_scripts_and_sources_are_reference_sites() {
        let files = vec![
            system(
                json!({"equipTypes": ["", "乳首"], "elements": ["", "炎"], "skillTypes": ["", "特殊"]}),
            ),
            (
                "CommonEvents.json".into(),
                "data/CommonEvents.json".into(),
                json!([null, {"list": [{"code": 355, "parameters": ["if (x === '特殊') {}"]}]}]),
            ),
        ];
        let plugins = "var $plugins =\n[\n{\"name\":\"CPM\",\"status\":true,\"description\":\"\",\"parameters\":{\"PictureList\":\"[\\\"{\\\\\\\"Name\\\\\\\":\\\\\\\"乳首←\\\\\\\"}\\\"]\"}},\n{\"name\":\"OffPlugin\",\"status\":false,\"description\":\"\",\"parameters\":{\"p\":\"炎\"}}\n];\n";
        let report = scan(&CoverageInput {
            data_files: &files,
            plugins_js: Some(plugins),
            plugin_sources: &[("Lit".into(), "const e = '炎';".into())],
            core_sources: &[],
        });
        let equip = category(&report, "system.equip_types");
        assert_eq!(equip.state, CategoryState::Blocked);
        assert_eq!(equip.references[0].site, SiteKind::PluginParam);
        assert_eq!(equip.references[0].location, "CPM::PictureList.0.Name");
        // Disabled plugins have no runtime effect: their params are not sites.
        let elements = category(&report, "system.elements");
        assert_eq!(elements.reference_count, 1);
        assert_eq!(elements.references[0].site, SiteKind::PluginSource);
        assert_eq!(
            category(&report, "system.skill_types").references[0].site,
            SiteKind::Script
        );
    }

    #[test]
    fn map_display_names_are_offered_and_core_warnings_reported() {
        let files = vec![
            (
                "Map001.json".into(),
                "data/Map001.json".into(),
                json!({"displayName": "始まりの村", "events": []}),
            ),
            (
                "MapInfos.json".into(),
                "data/MapInfos.json".into(),
                json!([null]),
            ),
            (
                "CommonEvents.json".into(),
                "data/CommonEvents.json".into(),
                json!([null, {"list": [{"code": 355, "parameters": ["$gameActors.actor(1).name() == '恵'"]}]}]),
            ),
        ];
        let report = scan(&CoverageInput {
            data_files: &files,
            plugins_js: None,
            plugin_sources: &[],
            core_sources: &[CoreSource {
                segment_kind: "actor_name".into(),
                source: "恵".into(),
            }],
        });
        let maps = category(&report, "map.display_name");
        assert_eq!(maps.state, CategoryState::Verified);
        assert_eq!(maps.candidates[0].key, "/displayName");
        assert_eq!(maps.candidates[0].kind, SegmentKind::MapDisplayName);
        assert_eq!(report.core_warnings.len(), 1);
        assert_eq!(report.core_warnings[0].references[0].site, SiteKind::Script);
    }

    #[test]
    fn unreadable_plugins_js_is_reported_not_guessed() {
        let report = scan(&CoverageInput {
            data_files: &[],
            plugins_js: Some("var $plugins = [ broken"),
            plugin_sources: &[],
            core_sources: &[],
        });
        assert!(report.plugins_unreadable.is_some());
    }
}
