# fileslug

Filename-aware slug generator for Rust. Converts messy filenames into clean, shell-safe slugs while preserving extensions, dotfiles, and version numbers.

Unlike URL slug libraries (which destroy `.tar.gz` → `tar-gz`), fileslug understands filenames.

## Usage

```rust
use fileslug::{slugify, SlugifyOptions, Style};

let opts = SlugifyOptions::default(); // kebab-case, transliterate unicode
assert_eq!(slugify("Café Résumé (Final).pdf", &opts), "cafe-resume-final.pdf");
assert_eq!(slugify(".env.local", &opts), ".env.local");
assert_eq!(slugify("archive.tar.gz", &opts), "archive.tar.gz");
assert_eq!(slugify("app-1.2.3.dmg", &opts), "app-1.2.3.dmg");

// Snake case
let snake = SlugifyOptions { style: Style::Snake, ..Default::default() };
assert_eq!(slugify("My Cool File.txt", &snake), "my_cool_file.txt");

// camelCase
let camel = SlugifyOptions { style: Style::Camel, ..Default::default() };
assert_eq!(slugify("my cool file.txt", &camel), "myCoolFile.txt");

// Keep unicode (skip transliteration)
let unicode = SlugifyOptions { keep_unicode: true, ..Default::default() };
assert_eq!(slugify("Café.txt", &unicode), "café.txt");
```

## Features

- **Extension preservation** — `.txt`, `.tar.gz`, `.tar.bz2` etc. never modified
- **Dotfile awareness** — `.gitignore`, `.env` returned as-is
- **Version number preservation** — `1.2.3` dots kept intact
- **Unicode transliteration** — via `any_ascii` (or keep-unicode to skip)
- **Three styles** — kebab-case (default), snake_case, camelCase
- **Zero-copy for dotfiles** — returns `Cow::Borrowed` when no changes needed
