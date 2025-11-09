# i18n-sync

A command-line utility for automatically syncing translation keys in rust_i18n projects.

## What it does

- Scans your Rust source code for `t!("key")` and `i18n!("key")` macros
- Checks your YAML locale files for missing translation keys
- Automatically adds missing keys with empty values
- Keeps all your locale files in sync

## Installation

```bash
cargo install i18n-sync
```

## Usage

Basic usage with default settings:
```bash
i18n-sync --locale-dir ./locales
```

Scan specific source directory:
```bash
i18n-sync --path ./src --locale-dir ./locales --default-locale en
```

With short flags:
```bash
i18n-sync -l ./locales -d en
```

## Options

- `-p, --path <PATH>` - Root path to scan for Rust files (default: ".")
- `-l, --locale-dir <DIR>` - Directory containing locale YAML files (required)
- `-d, --default-locale <LOCALE>` - Default locale (default: "en")

## Example

Before running i18n-sync:
- Your code has: `t!("hello")` and `t!("new_key")`
- Your `en.yml` only has: `hello: "Hello"`

After running i18n-sync, `en.yml` becomes:
```yaml
hello: "Hello"
new_key: ""
```

All other locale files (like `fr.yml`, `es.yml`) will also get the new key with empty values.

## Project structure

Your project should look like:
```
your_project/
├── src/
│   └── *.rs (with t! macros)
└── locales/
    ├── en.yml
    ├── fr.yml
    └── es.yml
```

## License

MIT
