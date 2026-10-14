//! Export integrity gate for RPG Maker MV/MZ.
//!
//! Every file an export produces is compared with the pristine original it was
//! derived from. The only differences allowed are the edits the export
//! *planned* (a JSON pointer → final string value). Anything else — a changed
//! number, a vanished key, a string the plan never mentioned — means an
//! injector bug and aborts the export before the archive is written.

use std::collections::HashMap;
use std::fmt;

use serde::Serialize;
use serde_json::Value;

/// Issues listed per file before the gate stops collecting (the export is
/// rejected either way; this only bounds the error message).
const MAX_ISSUES_PER_FILE: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum IntegrityIssue {
    /// The output (or original) does not parse.
    Unparseable { file: String, reason: String },
    /// A key, array length, type or non-string value differs.
    StructureChanged {
        file: String,
        pointer: String,
        reason: String,
    },
    /// A string changed that no planned edit covers.
    UnplannedChange { file: String, pointer: String },
    /// A planned edit is absent from the output.
    PlannedValueMissing { file: String, pointer: String },
}

impl fmt::Display for IntegrityIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unparseable { file, reason } => {
                write!(f, "{file}: output does not parse ({reason})")
            }
            Self::StructureChanged {
                file,
                pointer,
                reason,
            } => write!(f, "{file} {pointer}: structure changed ({reason})"),
            Self::UnplannedChange { file, pointer } => {
                write!(f, "{file} {pointer}: unplanned text change")
            }
            Self::PlannedValueMissing { file, pointer } => {
                write!(
                    f,
                    "{file} {pointer}: planned translation missing from output"
                )
            }
        }
    }
}

/// Verify that `output` differs from `original` exactly by `planned` edits.
pub fn verify_json_edit(
    file: &str,
    original: &str,
    output: &[u8],
    planned: &HashMap<String, String>,
) -> Vec<IntegrityIssue> {
    let parse = |bytes: &[u8]| serde_json::from_slice::<Value>(bytes);
    let before = match parse(original.as_bytes()) {
        Ok(value) => value,
        Err(error) => {
            return vec![IntegrityIssue::Unparseable {
                file: file.to_string(),
                reason: format!("original: {error}"),
            }]
        }
    };
    let after = match parse(output) {
        Ok(value) => value,
        Err(error) => {
            return vec![IntegrityIssue::Unparseable {
                file: file.to_string(),
                reason: error.to_string(),
            }]
        }
    };

    let mut walker = Walker {
        file,
        planned,
        issues: Vec::new(),
    };
    walker.walk(&before, &after, &mut String::new());
    for (pointer, value) in planned {
        if walker.issues.len() >= MAX_ISSUES_PER_FILE {
            break;
        }
        if after.pointer(pointer).and_then(Value::as_str) != Some(value.as_str()) {
            walker.issues.push(IntegrityIssue::PlannedValueMissing {
                file: file.to_string(),
                pointer: pointer.clone(),
            });
        }
    }
    walker.issues
}

struct Walker<'a> {
    file: &'a str,
    planned: &'a HashMap<String, String>,
    issues: Vec<IntegrityIssue>,
}

impl Walker<'_> {
    fn structure(&mut self, pointer: &str, reason: impl Into<String>) {
        if self.issues.len() < MAX_ISSUES_PER_FILE {
            self.issues.push(IntegrityIssue::StructureChanged {
                file: self.file.to_string(),
                pointer: pointer.to_string(),
                reason: reason.into(),
            });
        }
    }

    fn walk(&mut self, before: &Value, after: &Value, pointer: &mut String) {
        if self.issues.len() >= MAX_ISSUES_PER_FILE {
            return;
        }
        match (before, after) {
            (Value::Object(a), Value::Object(b)) => {
                if a.len() != b.len() || a.keys().any(|key| !b.contains_key(key)) {
                    self.structure(pointer, "object keys differ");
                    return;
                }
                for (key, value) in a {
                    let len = pointer.len();
                    pointer.push('/');
                    pointer.push_str(&key.replace('~', "~0").replace('/', "~1"));
                    self.walk(value, &b[key], pointer);
                    pointer.truncate(len);
                }
            }
            (Value::Array(a), Value::Array(b)) => {
                if a.len() != b.len() {
                    self.structure(pointer, format!("array length {} → {}", a.len(), b.len()));
                    return;
                }
                for (index, (left, right)) in a.iter().zip(b).enumerate() {
                    let len = pointer.len();
                    pointer.push('/');
                    pointer.push_str(&index.to_string());
                    self.walk(left, right, pointer);
                    pointer.truncate(len);
                }
            }
            (Value::String(a), Value::String(b)) => {
                if a != b && self.planned.get(pointer.as_str()) != Some(b) {
                    self.issues.push(IntegrityIssue::UnplannedChange {
                        file: self.file.to_string(),
                        pointer: pointer.clone(),
                    });
                }
            }
            (a, b) if std::mem::discriminant(a) == std::mem::discriminant(b) => {
                if a != b {
                    self.structure(pointer, "non-text value changed");
                }
            }
            _ => self.structure(pointer, "value type changed"),
        }
    }
}

/// Render a bounded, human-readable summary for an export error.
pub fn summarize(issues: &[IntegrityIssue]) -> String {
    const SHOWN: usize = 10;
    let mut lines: Vec<String> = issues.iter().take(SHOWN).map(ToString::to_string).collect();
    if issues.len() > SHOWN {
        lines.push(format!("… and {} more", issues.len() - SHOWN));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn planned_edits_pass() {
        let original = r#"{"a":"勇者","list":[{"t":"はい"},{"t":"x"}],"n":1}"#;
        let output = br#"{"a":"Hero","list":[{"t":"Yes"},{"t":"x"}],"n":1}"#;
        let issues = verify_json_edit(
            "Map.json",
            original,
            output,
            &plan(&[("/a", "Hero"), ("/list/0/t", "Yes")]),
        );
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn unplanned_text_and_structure_changes_are_caught() {
        let original = r#"{"a":"勇者","b":"剣","n":1,"arr":[1,2],"k":"x"}"#;
        let output = br#"{"a":"Hero","b":"Sword","n":2,"arr":[1],"k":1}"#;
        let issues = verify_json_edit("X.json", original, output, &plan(&[("/a", "Hero")]));
        assert!(issues.contains(&IntegrityIssue::UnplannedChange {
            file: "X.json".into(),
            pointer: "/b".into()
        }));
        assert!(issues.iter().any(
            |i| matches!(i, IntegrityIssue::StructureChanged { pointer, .. } if pointer == "/n")
        ));
        assert!(issues.iter().any(
            |i| matches!(i, IntegrityIssue::StructureChanged { pointer, .. } if pointer == "/arr")
        ));
        assert!(issues.iter().any(
            |i| matches!(i, IntegrityIssue::StructureChanged { pointer, .. } if pointer == "/k")
        ));
    }

    #[test]
    fn missing_planned_value_and_key_changes_are_caught() {
        let issues = verify_json_edit(
            "X.json",
            r#"{"a":"勇者","b":"剣"}"#,
            r#"{"a":"勇者","c":"剣"}"#.as_bytes(),
            &plan(&[("/a", "Hero")]),
        );
        assert!(issues
            .iter()
            .any(|i| matches!(i, IntegrityIssue::StructureChanged { .. })));
        assert!(issues.contains(&IntegrityIssue::PlannedValueMissing {
            file: "X.json".into(),
            pointer: "/a".into()
        }));
    }

    #[test]
    fn pointer_escaping_matches_rfc6901() {
        let issues = verify_json_edit(
            "X.json",
            r#"{"a/b":{"c~d":"元"}}"#,
            br#"{"a/b":{"c~d":"New"}}"#,
            &plan(&[("/a~1b/c~0d", "New")]),
        );
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn unparseable_output_is_reported() {
        let issues = verify_json_edit("X.json", "{}", b"{", &HashMap::new());
        assert!(matches!(
            issues.as_slice(),
            [IntegrityIssue::Unparseable { .. }]
        ));
    }
}
