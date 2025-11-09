use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
struct LocaleFile {
    #[serde(flatten)]
    translations: HashMap<String, String>,
}

pub fn update_locale_files(
    locale_dir: &str,
    translation_keys: &[String],
    default_locale: &str,
) -> anyhow::Result<()> {
    let locale_path = Path::new(locale_dir);

    if !locale_path.exists() {
        fs::create_dir_all(locale_path)?;
    }

    let default_locale_file = locale_path.join(format!("{}.yml", default_locale));
    let mut existing_translations = if default_locale_file.exists() {
        load_locale_file(&default_locale_file)?
    } else {
        HashMap::new()
    };

    let mut added_count = 0;
    for key in translation_keys {
        if !existing_translations.contains_key(key) {
            existing_translations.insert(key.clone(), String::new());
            added_count += 1;
        }
    }

    if added_count > 0 {
        save_locale_file(&default_locale_file, &existing_translations)?;
        println!(
            "Added {} new keys to {}",
            added_count,
            default_locale_file.display()
        );
    }

    for entry in fs::read_dir(locale_path)? {
        let entry = entry?;
        let path = entry.path();

        if path
            .extension()
            .map_or(false, |ext| ext == "yml" || ext == "yaml")
        {
            if path
                .file_stem()
                .map_or(false, |name| name == default_locale)
            {
                continue;
            }

            let mut locale_translations = load_locale_file(&path)?;
            let mut locale_added_count = 0;

            for key in translation_keys {
                if !locale_translations.contains_key(key) {
                    locale_translations.insert(key.clone(), String::new());
                    locale_added_count += 1;
                }
            }

            if locale_added_count > 0 {
                save_locale_file(&path, &locale_translations)?;
                println!(
                    "Added {} new keys to {}",
                    locale_added_count,
                    path.display()
                );
            }
        }
    }

    Ok(())
}

fn load_locale_file(path: &Path) -> anyhow::Result<HashMap<String, String>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let locale_data: LocaleFile = serde_yaml::from_str(&content).unwrap_or_else(|_| LocaleFile {
        translations: HashMap::new(),
    });

    Ok(locale_data.translations)
}

fn save_locale_file(path: &Path, translations: &HashMap<String, String>) -> anyhow::Result<()> {
    let mut sorted_translations: Vec<(&String, &String)> = translations.iter().collect();
    sorted_translations.sort_by(|a, b| a.0.cmp(b.0));

    let locale_data = LocaleFile {
        translations: translations.clone(),
    };

    let yaml_content = serde_yaml::to_string(&locale_data)?;
    let mut file = File::create(path)?;
    file.write_all(yaml_content.as_bytes())?;

    Ok(())
}
