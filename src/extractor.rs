use regex::Regex;
use std::fs;
use walkdir::WalkDir;

pub fn extract_translation_keys(root_path: &str) -> anyhow::Result<Vec<String>> {
    let mut keys = Vec::new();
    let t_macro_regex = Regex::new(r#"t!\("([^"]+)"\)"#)?;
    let i18n_macro_regex = Regex::new(r#"i18n!\("([^"]+)"\)"#)?;

    for entry in WalkDir::new(root_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(path)?;

            for cap in t_macro_regex.captures_iter(&content) {
                if let Some(key) = cap.get(1) {
                    keys.push(key.as_str().to_string());
                }
            }

            for cap in i18n_macro_regex.captures_iter(&content) {
                if let Some(key) = cap.get(1) {
                    keys.push(key.as_str().to_string());
                }
            }
        }
    }

    keys.sort();
    keys.dedup();
    Ok(keys)
}
