# slugr

[![Crates.io](https://img.shields.io/crates/v/slugr)](https://crates.io/crates/slugr)
[![Crates.io](https://img.shields.io/crates/v/fileslug)](https://crates.io/crates/fileslug)

Rename files and directories to clean, URL and shell-friendly slugs.

```
$ slugr "My Résumé (Final).pdf" "Photo 2024_01.JPG"
slugr: dry-run mode (use -x to execute)
My Résumé (Final).pdf -> my-resume-final.pdf
Photo 2024_01.JPG -> photo-2024-01.JPG
```

## Install

**Cargo** (requires [Rust](https://rustup.rs/)):

```bash
cargo install slugr
```

**Homebrew** (macOS):

```bash
brew install vmunix/tap/slugr
```

**Pre-built binaries** are available on the [releases page](https://github.com/vmunix/slugr/releases) for macOS (Intel & Apple Silicon) and Linux (x86_64 & ARM64).

## Usage

```
slugr [OPTIONS] [FILES]...
```

Dry-run is the default — slugr shows you what it *would* rename without touching anything. Pass `-x` to execute.

```bash
# Preview renames (dry-run)
slugr *.pdf

# Actually rename
slugr -x *.pdf

# Recurse into directories
slugr -rx my-project/

# Pipe from find
find . -name "*.txt" | slugr -x

# Interactive — prompt before each rename
slugr -ix *.jpg
```

## Options

| Flag | Long | Description |
|------|------|-------------|
| `-x` | `--execute` | Actually perform renames (default is dry-run) |
| `-r` | `--recursive` | Recurse into directories |
| `-v` | `--verbose` | Print each rename operation |
| `-i` | `--interactive` | Prompt before each rename |
| | `--clobber` | Allow overwriting existing files (default: no-clobber) |
| | `--snake` | Use `snake_case` instead of `kebab-case` |
| | `--camel` | Use `camelCase` instead of `kebab-case` |
| | `--keep-unicode` | Preserve unicode characters, only normalize separators |
| | `--pipe` | Pipe mode: read text from stdin, write slugified output to stdout |
| | `--raw` | Treat input as raw text, not filenames (requires `--pipe`) |

## Pipe mode

slugr can also slugify arbitrary text without touching the filesystem. Use `--pipe` to read from stdin and write slugified output to stdout:

```bash
# Slugify filenames (preserves extensions)
echo "My Résumé (Final).pdf" | slugr --pipe
# → my-resume-final.pdf

# Slugify raw text (no filename handling)
echo "My Blog Post Title!" | slugr --pipe --raw
# → my-blog-post-title

# Works with style flags
echo "My Blog Post" | slugr --pipe --raw --snake
# → my_blog_post

# Multiple lines
printf "Café Résumé\nHello World\n" | slugr --pipe --raw
# → cafe-resume
# → hello-world
```

By default, `--pipe` uses filename-aware slugification (preserving extensions and dotfiles). Add `--raw` to treat input as plain text — useful for generating URL slugs, identifiers, or clean strings.

## What it does

slugr takes messy filenames and makes them clean:

| Before | After |
|--------|-------|
| `My Cool File.txt` | `my-cool-file.txt` |
| `Café Résumé.pdf` | `cafe-resume.pdf` |
| `Report (Final) [2024].txt` | `report-final-2024.txt` |
| `file@name#with$symbols.txt` | `file-name-with-symbols.txt` |
| `too   many   spaces.txt` | `too-many-spaces.txt` |
| `My Archive.tar.gz` | `my-archive.tar.gz` |
| `app v1.2.3 release.zip` | `app-v1.2.3-release.zip` |

### Styles

**kebab-case** (default):
```
My Cool File.txt → my-cool-file.txt
```

**snake_case** (`--snake`):
```
My Cool File.txt → my_cool_file.txt
```

**camelCase** (`--camel`):
```
My Cool File.txt → myCoolFile.txt
```

### Unicode handling

By default, unicode characters are transliterated to ASCII:

```
Café Résumé.txt → cafe-resume.txt
你好世界.txt    → ni-hao-shi-jie.txt
```

With `--keep-unicode`, unicode letters are preserved and only separators are normalized:

```
Café Résumé.txt → café-résumé.txt
```

### Things slugr handles correctly

- **Dotfiles** — `.gitignore` and `.env` are left alone
- **Dotfiles with extensions** — `.env.local` stays as `.env.local`
- **Compound extensions** — `.tar.gz`, `.tar.bz2`, `.tar.xz`, `.tar.zst` are preserved
- **Extensions are never slugified** — only the base name is modified
- **Version numbers preserved** — `v1.2.3` stays `v1.2.3`, dots in versions aren't treated as separators
- **Collisions** — if the target name already exists, slugr appends `-2`, `-3`, etc. instead of overwriting
- **Case-only renames** — `README.txt` → `readme.txt` works correctly on case-insensitive filesystems (macOS)
- **Recursive renames** — children are renamed before parents so directory paths stay valid
- **Shell metacharacters** — `$(echo pwned).txt` becomes `echo-pwned.txt`, not a security hole