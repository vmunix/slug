mod cli;
mod rename;
mod walk;

#[cfg(test)]
mod fixtures;

use std::io::{self, BufRead, IsTerminal, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use cli::Cli;
use rename::{rename_file, RenameResult};
use fileslug::{slugify, slugify_string, SlugifyOptions};
use walk::collect_paths;

/// Pipe mode: read lines from stdin, slugify each, write to stdout.
fn run_pipe(options: &SlugifyOptions, raw: bool) -> ExitCode {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("slugr: read error: {e}");
                return ExitCode::FAILURE;
            }
        };
        if line.is_empty() {
            continue;
        }
        let slugified = if raw {
            slugify_string(&line, options)
        } else {
            slugify(&line, options)
        };
        if slugified.is_empty() {
            eprintln!("slugr: warning: '{line}' slugifies to empty");
            continue;
        }
        if writeln!(out, "{slugified}").is_err() {
            return ExitCode::FAILURE;
        }
    }
    if out.flush().is_err() {
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let style = args.style();

    let options = SlugifyOptions {
        style,
        keep_unicode: args.keep_unicode,
    };

    if args.pipe {
        return run_pipe(&options, args.raw);
    }

    let dry_run = !args.execute;
    let no_clobber = !args.clobber;

    let input_paths: Vec<PathBuf> = if !args.files.is_empty() {
        args.files
    } else if !io::stdin().is_terminal() {
        io::stdin()
            .lock()
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(PathBuf::from)
            .collect()
    } else {
        eprintln!("slugr: no files specified");
        return ExitCode::FAILURE;
    };

    let paths = collect_paths(&input_paths, args.recursive);

    if dry_run {
        eprintln!("slugr: dry-run mode (use -x to execute)");
    }

    let mut had_error = false;

    for path in &paths {
        let filename = match path.file_name() {
            Some(f) => f.to_string_lossy().to_string(),
            None => continue,
        };

        let new_name = slugify(&filename, &options);

        // Empty/dot slugs would resolve to the parent directory
        if new_name.is_empty() || *new_name == *"." || *new_name == *".." {
            eprintln!("slugr: cannot rename '{}': slugified name is invalid", path.display());
            had_error = true;
            continue;
        }

        let parent = path.parent().unwrap_or(std::path::Path::new("."));
        let target = parent.join(&*new_name);

        if args.interactive && path != &target {
            eprint!("slugr: rename '{}' -> '{}'? [y/N] ", path.display(), target.display());
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).ok();
            if !answer.trim().eq_ignore_ascii_case("y") {
                continue;
            }
        }

        let result = rename_file(path, &target, no_clobber, dry_run);

        match &result {
            RenameResult::Renamed { from, to } => {
                if dry_run || args.verbose {
                    println!("{} -> {}", from.display(), to.display());
                }
            }
            RenameResult::Skipped(_) => {}
            RenameResult::Failed { path, error } => {
                eprintln!("slugr: error renaming '{}': {}", path.display(), error);
                had_error = true;
            }
        }
    }

    if had_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
