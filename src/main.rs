mod cli;
mod extractor;
mod locale_manager;

use anyhow::Result;

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    let root_path = matches.get_one::<String>("path").unwrap();
    let locale_dir = matches.get_one::<String>("locale-dir").unwrap();
    let default_locale = matches.get_one::<String>("default-locale").unwrap();

    println!("Scanning Rust files in: {}", root_path);
    println!("Locale directory: {}", locale_dir);
    println!("Default locale: {}", default_locale);

    let translation_keys = extractor::extract_translation_keys(root_path)?;
    println!("Found {} translation keys in code", translation_keys.len());

    locale_manager::update_locale_files(locale_dir, &translation_keys, default_locale)?;

    Ok(())
}
