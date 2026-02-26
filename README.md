# slugr

Rename files and directories to clean, URL and shell friendly slugs.

```
$ slugr "My Résumé (Final).pdf" "Photo 2024_01.JPG"
slugr: dry-run mode (use -x to execute)
My Résumé (Final).pdf -> my-resume-final.pdf
Photo 2024_01.JPG -> photo-2024-01.JPG
```

## Install

**Homebrew (macOS):**

```bash
brew install vmunix/tap/slugr
```

**Cargo:**

```bash
cargo install --path slugr
```

**Pre-built binaries** are available on the [releases page](https://github.com/vmunix/slugr/releases) for macOS (Intel & Apple Silicon) and Linux (x86_64 & ARM64).

Or build from source:

```bash
cargo build --release
# binary is at target/release/slugr
```

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