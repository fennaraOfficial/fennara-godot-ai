use crate::app_layout::display_path;
use std::fs;
use std::path::Path;

const AGENTS_BLOCK: &str = include_str!("../../../templates/AGENTS.block.md");
const AGENTS_START: &str = "<!-- fennara-agents-start -->";
const AGENTS_END: &str = "<!-- fennara-agents-end -->";
const AI_GUIDANCE_FILES: &[(&[&str], &str)] = &[
    (
        &["addons", "fennara", "ai", "guidelines.md"],
        include_str!("../../../templates/fennara-guidelines.md"),
    ),
    (
        &["addons", "fennara", "ai", "index.md"],
        include_str!("../../../templates/fennara-ai/index.md"),
    ),
    (
        &["addons", "fennara", "ai", "visual-observation.md"],
        include_str!("../../../templates/fennara-ai/visual-observation.md"),
    ),
    (
        &["addons", "fennara", "ai", "runtime-observation.md"],
        include_str!("../../../templates/fennara-ai/runtime-observation.md"),
    ),
    (
        &["addons", "fennara", "ai", "operations.md"],
        include_str!("../../../templates/fennara-ai/operations.md"),
    ),
    (
        &["addons", "fennara", "ai", "clients", "cursor.md"],
        include_str!("../../../templates/fennara-ai/clients/cursor.md"),
    ),
];

pub fn write(project_dir: &Path) -> Result<(), String> {
    write_ai_guidance(project_dir)?;
    write_project_files(project_dir)
}

pub fn write_project_files(project_dir: &Path) -> Result<(), String> {
    update_agents(project_dir)?;
    update_gitignore_if_present(project_dir)
}

fn write_ai_guidance(project_dir: &Path) -> Result<(), String> {
    for (parts, template) in AI_GUIDANCE_FILES {
        let target = parts
            .iter()
            .fold(project_dir.to_path_buf(), |path, part| path.join(part));

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to create {}: {err}", display_path(parent)))?;
        }

        write_if_changed(&target, normalize_template(template).as_bytes())?;
    }
    Ok(())
}

fn update_agents(project_dir: &Path) -> Result<(), String> {
    let agents_path = project_dir.join("AGENTS.md");
    let block = normalize_template(AGENTS_BLOCK);
    let existing = fs::read_to_string(&agents_path).unwrap_or_default();
    let next = replace_or_append_block(&existing, &block)?;

    write_if_changed(&agents_path, next.as_bytes())
}

fn update_gitignore_if_present(project_dir: &Path) -> Result<(), String> {
    let gitignore_path = project_dir.join(".gitignore");
    if !gitignore_path.is_file() {
        return Ok(());
    }

    let existing = fs::read_to_string(&gitignore_path)
        .map_err(|err| format!("failed to read {}: {err}", display_path(&gitignore_path)))?;
    let already_ignored = existing
        .lines()
        .any(|line| matches!(line.trim(), ".fennara" | ".fennara/"));
    if already_ignored {
        return Ok(());
    }

    let mut next = ensure_single_trailing_newline(&existing);
    next.push_str(".fennara/\n");
    write_if_changed(&gitignore_path, next.as_bytes())
}

fn replace_or_append_block(existing: &str, block: &str) -> Result<String, String> {
    if existing.trim().is_empty() {
        return Ok(ensure_single_trailing_newline(block));
    }

    if let Some(start) = existing.find(AGENTS_START) {
        let Some(end_relative) = existing[start..].find(AGENTS_END) else {
            return Err(format!(
                "found {AGENTS_START} in AGENTS.md but could not find {AGENTS_END}"
            ));
        };
        let end = start + end_relative + AGENTS_END.len();
        let mut next = String::new();
        next.push_str(&existing[..start]);
        next.push_str(block);
        next.push_str(&existing[end..]);
        return Ok(ensure_single_trailing_newline(&next));
    }

    Ok(format!("{}\n\n{block}\n", existing.trim_end()))
}

fn normalize_template(template: &str) -> String {
    ensure_single_trailing_newline(template.trim())
}

fn ensure_single_trailing_newline(value: &str) -> String {
    format!("{}\n", value.trim_end())
}

fn write_if_changed(path: &Path, content: &[u8]) -> Result<(), String> {
    if fs::read(path).ok().as_deref() == Some(content) {
        return Ok(());
    }

    fs::write(path, content).map_err(|err| format!("failed to write {}: {err}", display_path(path)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_block_to_existing_agents_file() {
        let next = replace_or_append_block("# Existing\n", "BLOCK").unwrap();
        assert_eq!(next, "# Existing\n\nBLOCK\n");
    }

    #[test]
    fn replaces_existing_generated_block() {
        let existing =
            "before\n<!-- fennara-agents-start -->\nold\n<!-- fennara-agents-end -->\nafter\n";
        let next = replace_or_append_block(existing, "new").unwrap();
        assert_eq!(next, "before\nnew\nafter\n");
    }

    #[test]
    fn appends_fennara_to_existing_gitignore() {
        let temp =
            std::env::temp_dir().join(format!("fennara-guidance-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();
        let gitignore = temp.join(".gitignore");
        fs::write(&gitignore, "target/\n").unwrap();

        update_gitignore_if_present(&temp).unwrap();

        assert_eq!(
            fs::read_to_string(&gitignore).unwrap(),
            "target/\n.fennara/\n"
        );
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn new_agents_file_is_stable_after_first_write() {
        let block = format!("{AGENTS_START}\nBLOCK\n{AGENTS_END}\n");
        let first = replace_or_append_block("", &block).unwrap();
        let second = replace_or_append_block(&first, &block).unwrap();
        assert_eq!(first, block);
        assert_eq!(second, first);
    }

    #[test]
    fn project_files_do_not_modify_addon_guidance() {
        let temp = std::env::temp_dir().join(format!(
            "fennara-guidance-project-files-test-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&temp);
        let guidelines = temp.join("addons/fennara/ai/guidelines.md");
        let index = temp.join("addons/fennara/ai/index.md");
        fs::create_dir_all(guidelines.parent().unwrap()).unwrap();
        fs::write(&guidelines, "store addon content\n").unwrap();
        fs::write(&index, "store index\n").unwrap();

        write_project_files(&temp).unwrap();

        assert_eq!(
            fs::read_to_string(guidelines).unwrap(),
            "store addon content\n"
        );
        assert_eq!(fs::read_to_string(index).unwrap(), "store index\n");
        assert!(temp.join("AGENTS.md").is_file());
        let _ = fs::remove_dir_all(temp);
    }

    #[test]
    fn write_installs_every_ai_guidance_file() {
        let temp = std::env::temp_dir().join(format!(
            "fennara-guidance-all-files-test-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();

        write(&temp).unwrap();

        for (parts, template) in AI_GUIDANCE_FILES {
            let target = parts
                .iter()
                .fold(temp.to_path_buf(), |path, part| path.join(part));
            assert_eq!(
                fs::read_to_string(&target).unwrap(),
                normalize_template(template),
                "unexpected generated guidance at {}",
                target.display()
            );
        }
        let _ = fs::remove_dir_all(temp);
    }

    #[test]
    fn guidelines_stay_compact_and_index_routes_specialized_pages() {
        let guidelines = AI_GUIDANCE_FILES[0].1;
        assert!(
            guidelines.split_whitespace().count() <= 1_200,
            "always-read guidelines exceeded the 1,200-word budget"
        );

        let index = AI_GUIDANCE_FILES[1].1;
        for (parts, _) in &AI_GUIDANCE_FILES[2..] {
            let resource_path = format!("res://{}", parts.join("/"));
            assert!(
                index.contains(&resource_path),
                "index does not route to {resource_path}"
            );
        }
    }
}
