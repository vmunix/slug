# Pipe Mode: Slugify Arbitrary Text (Issue #2)

## Summary

Add a `--pipe` flag that switches slugr from file-rename mode to a stdin→stdout text filter. By default, pipe mode uses filename-aware slugification (preserving extensions, dotfiles). A `--raw` flag disables filename smarts and treats input as plain text.

## Two Modes

1. **Rename mode** (default) — takes file paths, renames on disk
2. **Pipe mode** (`--pipe`) — reads stdin line-by-line, writes slugified output to stdout, no filesystem interaction

## CLI Flags

- `--pipe` — switch to pipe mode
- `--raw` — only valid with `--pipe`; skip extension/dotfile handling, treat input as plain text
- `--pipe` conflicts with `-x` (`--execute`), `--clobber`, `-i` (`--interactive`), `-r` (`--recursive`)
- Style flags (`--snake`, `--camel`, `--keep-unicode`) work in both modes

## Library Changes (fileslug)

New public function `slugify_string()`:
- Same pipeline as `slugify()` but skips `split_extension()` — entire input is the "base"
- No extension preservation, no dotfile handling
- Version dot preservation, transliteration, bracket stripping, truncation all still apply
- Returns `Cow<str>` like `slugify()`

## Architecture (Approach A)

Keep `slugify()` and `slugify_string()` as separate functions in fileslug. The CLI dispatches to the appropriate one based on `--raw`.

## Data Flow (Pipe Mode)

```
stdin (line) → slugify() or slugify_string() → stdout (line)
```

## Error Handling

- Empty lines → output empty line
- Lines that slugify to empty string → output empty line, warn on stderr

## Examples

```bash
# Pipe mode, filename-aware (default)
echo "My Résumé (Final).pdf" | slugr --pipe
# → my-resume-final.pdf

# Pipe mode, raw text
echo "My Blog Post Title!" | slugr --pipe --raw
# → my-blog-post-title

# With style flags
echo "My Blog Post" | slugr --pipe --raw --snake
# → my_blog_post

# Multiple lines
printf "Café Résumé\nHello World\n" | slugr --pipe --raw
# → cafe-resume
# → hello-world
```
