//! The Agent Skills spec rules for one skill directory, with the messages of
//! the reference validator `skills-ref`
//! (<https://github.com/agentskills/agentskills/tree/main/skills-ref>).

use std::path::Path;

use unicode_normalization::UnicodeNormalization;

use super::frontmatter::{self, Node};

const MAX_NAME_LENGTH: usize = 64;
const MAX_DESCRIPTION_LENGTH: usize = 1024;
const MAX_COMPATIBILITY_LENGTH: usize = 500;

/// The frontmatter keys the spec allows, sorted.
const ALLOWED_FIELDS: [&str; 6] = [
    "allowed-tools",
    "compatibility",
    "description",
    "license",
    "metadata",
    "name",
];

/// Checks a skill directory and returns one message per problem.
pub fn validate(dir: &Path) -> Vec<String> {
    let Some(skill_md) = ["SKILL.md", "skill.md"]
        .iter()
        .map(|name| dir.join(name))
        .find(|path| path.exists())
    else {
        return vec!["Missing required file: SKILL.md".to_owned()];
    };
    let content = match std::fs::read(&skill_md) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(content) => content,
            Err(_) => return vec!["SKILL.md is not valid UTF-8".to_owned()],
        },
        Err(err) => return vec![format!("SKILL.md could not be read: {err}")],
    };
    // Python's read_text() turns every line ending into "\n".
    let content = content.replace("\r\n", "\n").replace('\r', "\n");
    match frontmatter::parse_skill(&content) {
        Ok(entries) => validate_metadata(&entries, dir),
        Err(err) => vec![err.to_string()],
    }
}

fn validate_metadata(entries: &[(String, Node)], dir: &Path) -> Vec<String> {
    let get = |key: &str| {
        entries
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, node)| node)
    };
    let mut errors = Vec::new();

    let mut extra: Vec<&str> = entries
        .iter()
        .map(|(key, _)| key.as_str())
        .filter(|key| !ALLOWED_FIELDS.contains(key))
        .collect();
    if !extra.is_empty() {
        extra.sort_unstable();
        let allowed: Vec<String> = ALLOWED_FIELDS.iter().map(|f| format!("'{f}'")).collect();
        errors.push(format!(
            "Unexpected fields in frontmatter: {}. Only [{}] are allowed.",
            extra.join(", "),
            allowed.join(", ")
        ));
    }

    match get("name") {
        None => errors.push("Missing required field in frontmatter: name".to_owned()),
        Some(name) => errors.extend(validate_name(name, dir)),
    }
    match get("description") {
        None => errors.push("Missing required field in frontmatter: description".to_owned()),
        Some(description) => errors.extend(validate_description(description)),
    }
    if let Some(compatibility) = get("compatibility") {
        match compatibility {
            Node::Scalar(value) => {
                let length = value.chars().count();
                if length > MAX_COMPATIBILITY_LENGTH {
                    errors.push(format!(
                        "Compatibility exceeds {MAX_COMPATIBILITY_LENGTH} character limit ({length} chars)"
                    ));
                }
            }
            _ => errors.push("Field 'compatibility' must be a string".to_owned()),
        }
    }
    errors
}

fn non_empty_string(node: &Node) -> Option<&str> {
    match node {
        Node::Scalar(value) if !value.trim().is_empty() => Some(value),
        _ => None,
    }
}

fn validate_name(node: &Node, dir: &Path) -> Vec<String> {
    let Some(name) = non_empty_string(node) else {
        return vec!["Field 'name' must be a non-empty string".to_owned()];
    };
    let name: String = name.trim().nfkc().collect();
    let mut errors = Vec::new();

    let length = name.chars().count();
    if length > MAX_NAME_LENGTH {
        errors.push(format!(
            "Skill name '{name}' exceeds {MAX_NAME_LENGTH} character limit ({length} chars)"
        ));
    }
    if name != name.to_lowercase() {
        errors.push(format!("Skill name '{name}' must be lowercase"));
    }
    if name.starts_with('-') || name.ends_with('-') {
        errors.push("Skill name cannot start or end with a hyphen".to_owned());
    }
    if name.contains("--") {
        errors.push("Skill name cannot contain consecutive hyphens".to_owned());
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        errors.push(format!(
            "Skill name '{name}' contains invalid characters. Only letters, digits, and hyphens are allowed."
        ));
    }

    let dir_name = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    if dir_name.nfkc().collect::<String>() != name {
        errors.push(format!(
            "Directory name '{dir_name}' must match skill name '{name}'"
        ));
    }
    errors
}

fn validate_description(node: &Node) -> Vec<String> {
    let Some(description) = non_empty_string(node) else {
        return vec!["Field 'description' must be a non-empty string".to_owned()];
    };
    let length = description.chars().count();
    if length > MAX_DESCRIPTION_LENGTH {
        return vec![format!(
            "Description exceeds {MAX_DESCRIPTION_LENGTH} character limit ({length} chars)"
        )];
    }
    Vec::new()
}
