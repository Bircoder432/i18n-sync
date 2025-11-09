use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("i18n-gen")
        .version("0.1.0")
        .about("Utility for generating and managing translation keys for rust_i18n")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Root path to scan for Rust files")
                .default_value("."),
        )
        .arg(
            Arg::new("locale-dir")
                .short('l')
                .long("locale-dir")
                .value_name("DIR")
                .help("Directory containing locale files")
                .required(true),
        )
        .arg(
            Arg::new("default-locale")
                .short('d')
                .long("default-locale")
                .value_name("LOCALE")
                .help("Default locale to use as reference")
                .default_value("en"),
        )
}
