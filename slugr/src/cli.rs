use std::path::PathBuf;

use clap::Parser;

use fileslug::Style;

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[command(
    name = "slugr",
    version,
    about = "Slugr — a filesystem-aware slug generator",
    long_about = "Renames files and directories by converting their names to clean, \
                  URL-friendly slugs. Dry-run by default; use -x to execute."
)]
pub struct Cli {
    /// Actually perform renames (default is dry-run)
    #[arg(short = 'x', long)]
    pub execute: bool,

    /// Print each rename operation
    #[arg(short, long)]
    pub verbose: bool,

    /// Allow overwriting existing files (default: no-clobber)
    #[arg(long)]
    pub clobber: bool,

    /// Prompt before each rename
    #[arg(short, long)]
    pub interactive: bool,

    /// Recurse into directories
    #[arg(short, long)]
    pub recursive: bool,

    /// Use `snake_case` instead of kebab-case
    #[arg(long, conflicts_with = "camel")]
    pub snake: bool,

    /// Use camelCase instead of kebab-case
    #[arg(long, conflicts_with = "snake")]
    pub camel: bool,

    /// Preserve unicode characters, only normalize separators
    #[arg(long)]
    pub keep_unicode: bool,

    /// Pipe mode: read text from stdin, write slugified output to stdout
    #[arg(long, conflicts_with_all = ["execute", "clobber", "interactive", "recursive"])]
    pub pipe: bool,

    /// Treat input as raw text, not filenames (skip extension handling). Requires --pipe
    #[arg(long, requires = "pipe")]
    pub raw: bool,

    /// Files and directories to rename
    pub files: Vec<PathBuf>,
}

impl Cli {
    pub fn style(&self) -> Style {
        match (self.snake, self.camel) {
            (true, _) => Style::Snake,
            (_, true) => Style::Camel,
            _ => Style::Kebab,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let args = Cli::parse_from(["slugr", "file.txt"]);
        assert!(!args.execute);
        assert!(!args.verbose);
        assert!(!args.clobber);
        assert!(!args.interactive);
        assert!(!args.recursive);
        assert!(!args.snake);
        assert!(!args.camel);
        assert!(!args.keep_unicode);
        assert!(!args.pipe);
        assert!(!args.raw);
        assert_eq!(args.files, vec![PathBuf::from("file.txt")]);
    }

    #[test]
    fn test_all_flags() {
        let args = Cli::parse_from([
            "slugr", "-x", "-v", "-i", "-r", "--snake", "--keep-unicode", "a.txt", "b.txt",
        ]);
        assert!(args.execute);
        assert!(args.verbose);
        assert!(args.interactive);
        assert!(args.recursive);
        assert!(args.snake);
        assert!(args.keep_unicode);
        assert_eq!(args.files, vec![PathBuf::from("a.txt"), PathBuf::from("b.txt")]);
    }

    #[test]
    fn test_clobber_opt_in() {
        let args = Cli::parse_from(["slugr", "--clobber", "file.txt"]);
        assert!(args.clobber);
    }

    #[test]
    fn test_snake_and_camel_conflict() {
        let result = Cli::try_parse_from(["slugr", "--snake", "--camel", "file.txt"]);
        assert!(result.is_err(), "should error when both --snake and --camel are set");
    }

    #[test]
    fn test_recursive_with_file_argument() {
        let args = Cli::parse_from(["slugr", "-r", "file.txt"]);
        assert!(args.recursive);
        assert_eq!(args.files, vec![PathBuf::from("file.txt")]);
    }

    #[test]
    fn test_pipe_flag() {
        let args = Cli::parse_from(["slugr", "--pipe"]);
        assert!(args.pipe);
        assert!(!args.raw);
    }

    #[test]
    fn test_pipe_with_raw() {
        let args = Cli::parse_from(["slugr", "--pipe", "--raw"]);
        assert!(args.pipe);
        assert!(args.raw);
    }

    #[test]
    fn test_raw_requires_pipe() {
        let result = Cli::try_parse_from(["slugr", "--raw", "file.txt"]);
        assert!(result.is_err(), "--raw without --pipe should error");
    }

    #[test]
    fn test_pipe_conflicts_with_execute() {
        let result = Cli::try_parse_from(["slugr", "--pipe", "-x"]);
        assert!(result.is_err(), "--pipe should conflict with -x");
    }

    #[test]
    fn test_pipe_conflicts_with_recursive() {
        let result = Cli::try_parse_from(["slugr", "--pipe", "-r"]);
        assert!(result.is_err(), "--pipe should conflict with -r");
    }

    #[test]
    fn test_pipe_conflicts_with_interactive() {
        let result = Cli::try_parse_from(["slugr", "--pipe", "-i"]);
        assert!(result.is_err(), "--pipe should conflict with -i");
    }

    #[test]
    fn test_pipe_conflicts_with_clobber() {
        let result = Cli::try_parse_from(["slugr", "--pipe", "--clobber"]);
        assert!(result.is_err(), "--pipe should conflict with --clobber");
    }

    #[test]
    fn test_pipe_with_style_flags() {
        let args = Cli::parse_from(["slugr", "--pipe", "--snake"]);
        assert!(args.pipe);
        assert!(args.snake);
    }
}
