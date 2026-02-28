# fileslug

Slug generator for Rust. Slugifies filenames and arbitrary text into clean, URL and shell-friendly strings.

Two entry points:

- **`slugify()`** — filename-aware: preserves extensions, dotfiles, compound extensions (`.tar.gz`), and version numbers
- **`slugify_string()`** — plain text: treats input as a raw string with no filename handling — use for URL slugs, identifiers, titles, etc.

## Usage

### Filenames

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

### Arbitrary text

```rust
use fileslug::{slugify_string, SlugifyOptions};

let opts = SlugifyOptions::default();
assert_eq!(slugify_string("My Blog Post Title!", &opts), "my-blog-post-title");
assert_eq!(slugify_string("Café Résumé", &opts), "cafe-resume");
```

## Features

- **Two modes** — filename-aware (`slugify`) and plain text (`slugify_string`)
- **Extension preservation** — `.txt`, `.tar.gz`, `.tar.bz2` etc. never modified
- **Dotfile awareness** — `.gitignore`, `.env` returned as-is
- **Version number preservation** — `1.2.3` dots kept intact
- **Unicode transliteration** — via `any_ascii` (or keep-unicode to skip)
- **Three styles** — kebab-case (default), snake_case, camelCase
- **Zero-copy for dotfiles** — returns `Cow::Borrowed` when no changes needed
