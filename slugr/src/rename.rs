use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use fileslug::split_extension;

/// Check if two paths refer to the same file (same inode on the same device).
/// Returns false if either path doesn't exist.
#[cfg(unix)]
fn same_file(a: &Path, b: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;
    match (fs::metadata(a), fs::metadata(b)) {
        (Ok(ma), Ok(mb)) => ma.dev() == mb.dev() && ma.ino() == mb.ino(),
        _ => false,
    }
}

#[cfg(not(unix))]
fn same_file(a: &Path, b: &Path) -> bool {
    a == b
}

/// The result of a rename operation.
#[derive(Debug)]
pub enum RenameResult {
    /// File was renamed from old to new path.
    Renamed { from: PathBuf, to: PathBuf },
    /// File was already clean, no rename needed.
    Skipped(#[allow(dead_code)] PathBuf),
    /// Rename failed with an error.
    Failed { path: PathBuf, error: io::Error },
}

/// Maximum number of collision suffixes to try before giving up.
const MAX_COLLISION_SUFFIX: u32 = 1_000;

/// Find a non-colliding target path, appending `-2`, `-3`, etc. if needed.
///
/// `source` is excluded from collision checks so that case-only renames
/// (e.g. `File.txt` → `file.txt`) don't falsely collide on case-insensitive
/// filesystems. Returns an error after 1,000 suffixes are exhausted.
pub fn safe_target(target: &Path, no_clobber: bool, source: Option<&Path>) -> io::Result<PathBuf> {
    let collides = |p: &Path| p.exists() && !source.is_some_and(|s| same_file(s, p));

    if !no_clobber || !collides(target) {
        return Ok(target.to_path_buf());
    }

    let filename = target.file_name().unwrap().to_string_lossy();
    let (base, ext) = split_extension(&filename);
    let parent = target.parent().unwrap_or(Path::new("."));

    let format_candidate = |n: u32| -> PathBuf {
        if base.is_empty() {
            // Pure dotfile (e.g. ".txt"): append suffix after name → .txt-2
            parent.join(format!("{ext}-{n}"))
        } else {
            parent.join(format!("{base}-{n}{ext}"))
        }
    };

    (2..=MAX_COLLISION_SUFFIX + 1)
        .map(format_candidate)
        .find(|candidate| !collides(candidate))
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("too many collisions for '{}'", target.display()),
            )
        })
}

/// Rename a single file/directory from `source` to `target`.
/// If `no_clobber` is true and `target` exists, appends a numeric suffix.
/// If `dry_run` is true, does not perform the rename.
/// Handles case-only renames on case-insensitive filesystems (macOS).
#[must_use]
pub fn rename_file(source: &Path, target: &Path, no_clobber: bool, dry_run: bool) -> RenameResult {
    if source == target {
        return RenameResult::Skipped(source.to_path_buf());
    }

    // Case-only rename (same inode, different name): skip safe_target to avoid
    // false collision on case-insensitive filesystems
    let is_case_only = same_file(source, target);

    let final_target = if is_case_only {
        target.to_path_buf()
    } else {
        match safe_target(target, no_clobber, Some(source)) {
            Ok(t) => t,
            Err(e) => {
                return RenameResult::Failed {
                    path: source.to_path_buf(),
                    error: e,
                };
            }
        }
    };

    if dry_run {
        return RenameResult::Renamed {
            from: source.to_path_buf(),
            to: final_target,
        };
    }

    match fs::rename(source, &final_target) {
        Ok(()) => RenameResult::Renamed {
            from: source.to_path_buf(),
            to: final_target,
        },
        Err(e) => RenameResult::Failed {
            path: source.to_path_buf(),
            error: e,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_safe_target_no_collision() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("clean-file.txt");
        assert_eq!(safe_target(&target, true, None).unwrap(), target);
    }

    #[test]
    fn test_safe_target_with_collision() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("file.txt");
        fs::write(&target, "existing").unwrap();
        let result = safe_target(&target, true, None).unwrap();
        assert_eq!(result, dir.path().join("file-2.txt"));
    }

    #[test]
    fn test_safe_target_multiple_collisions() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("file.txt"), "a").unwrap();
        fs::write(dir.path().join("file-2.txt"), "b").unwrap();
        fs::write(dir.path().join("file-3.txt"), "c").unwrap();
        let result = safe_target(&dir.path().join("file.txt"), true, None).unwrap();
        assert_eq!(result, dir.path().join("file-4.txt"));
    }

    #[test]
    fn test_safe_target_compound_ext() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("archive.tar.gz");
        fs::write(&target, "existing").unwrap();
        let result = safe_target(&target, true, None).unwrap();
        assert_eq!(result, dir.path().join("archive-2.tar.gz"));
    }

    #[test]
    fn test_safe_target_no_clobber_off() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("file.txt");
        fs::write(&target, "existing").unwrap();
        assert_eq!(safe_target(&target, false, None).unwrap(), target);
    }

    #[test]
    fn test_rename_file_basic() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("My File.txt");
        fs::write(&source, "hello").unwrap();
        let target = dir.path().join("my-file.txt");

        let result = rename_file(&source, &target, true, false);
        match result {
            RenameResult::Renamed { from, to } => {
                assert_eq!(from, source);
                assert_eq!(to, target);
            }
            other => panic!("expected Renamed, got {other:?}"),
        }
        assert!(!source.exists());
        assert!(target.exists());
        assert_eq!(fs::read_to_string(&target).unwrap(), "hello");
    }

    #[test]
    fn test_rename_file_dry_run() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("My File.txt");
        fs::write(&source, "hello").unwrap();
        let target = dir.path().join("my-file.txt");

        let result = rename_file(&source, &target, true, true);
        match result {
            RenameResult::Renamed { from, to } => {
                assert_eq!(from, source);
                assert_eq!(to, target);
            }
            other => panic!("expected Renamed, got {other:?}"),
        }
        // File should NOT have moved
        assert!(source.exists());
        assert!(!target.exists());
    }

    #[test]
    fn test_rename_file_same_name() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("already-clean.txt");
        fs::write(&source, "hello").unwrap();

        let result = rename_file(&source, &source, true, false);
        match result {
            RenameResult::Skipped(path) => assert_eq!(path, source),
            other => panic!("expected Skipped, got {other:?}"),
        }
        assert!(source.exists());
    }

    /// On macOS (case-insensitive FS), renaming File.txt → file.txt should work
    /// as a case-only change, not produce file-2.txt.
    #[test]
    #[cfg(target_os = "macos")]
    fn test_rename_case_only_change() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("File.txt");
        fs::write(&source, "hello").unwrap();
        let target = dir.path().join("file.txt");

        let result = rename_file(&source, &target, true, false);
        match result {
            RenameResult::Renamed { from, to } => {
                assert_eq!(from, source);
                assert_eq!(to, target);
            }
            other => panic!("expected Renamed, got {other:?}"),
        }
        // On case-insensitive FS, file.txt should exist with the new casing
        assert!(target.exists());
        assert_eq!(fs::read_to_string(&target).unwrap(), "hello");
        // Verify actual filename on disk is the new casing
        let entries: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        assert!(entries.contains(&"file.txt".to_string()), "expected file.txt on disk, got {entries:?}");
    }

    /// On macOS, `safe_target` should not treat the source itself as a collision
    /// when source and target differ only in case.
    #[test]
    #[cfg(target_os = "macos")]
    fn test_safe_target_case_insensitive_self() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("File.txt");
        fs::write(&source, "hello").unwrap();
        let target = dir.path().join("file.txt");

        let result = safe_target(&target, true, Some(&source)).unwrap();
        // Should return file.txt, not file-2.txt
        assert_eq!(result, target);
    }

    #[test]
    fn test_rename_source_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("nonexistent.txt");
        let target = dir.path().join("target.txt");

        let result = rename_file(&source, &target, true, false);
        match result {
            RenameResult::Failed { path, error } => {
                assert_eq!(path, source);
                assert!(!error.to_string().is_empty());
            }
            other => panic!("expected Failed, got {other:?}"),
        }
    }

    #[test]
    fn test_rename_readonly_directory() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("readonly");
        fs::create_dir(&sub).unwrap();
        let source = sub.join("file.txt");
        fs::write(&source, "hello").unwrap();
        let target = sub.join("renamed.txt");

        // Make directory read-only
        fs::set_permissions(&sub, fs::Permissions::from_mode(0o555)).unwrap();

        let result = rename_file(&source, &target, true, false);

        // Restore permissions for cleanup
        fs::set_permissions(&sub, fs::Permissions::from_mode(0o755)).unwrap();

        match result {
            RenameResult::Failed { path, error } => {
                assert_eq!(path, source);
                assert!(!error.to_string().is_empty());
            }
            other => panic!("expected Failed, got {other:?}"),
        }
    }

    #[test]
    #[cfg(unix)]
    fn test_rename_symlink_not_followed() {
        let dir = tempfile::tempdir().unwrap();
        let real_file = dir.path().join("real.txt");
        fs::write(&real_file, "real content").unwrap();
        let link = dir.path().join("My Link.txt");
        std::os::unix::fs::symlink(&real_file, &link).unwrap();
        let target = dir.path().join("my-link.txt");

        let result = rename_file(&link, &target, true, false);
        match result {
            RenameResult::Renamed { from, to } => {
                assert_eq!(from, link);
                assert_eq!(to, target);
            }
            other => panic!("expected Renamed, got {other:?}"),
        }
        // Symlink itself was renamed
        assert!(!link.exists());
        assert!(target.symlink_metadata().unwrap().file_type().is_symlink());
        // Original file untouched
        assert!(real_file.exists());
        assert_eq!(fs::read_to_string(&real_file).unwrap(), "real content");
    }

    #[test]
    fn test_safe_target_dotfile_collision() {
        let dir = tempfile::tempdir().unwrap();
        // .txt is treated as a dotfile with no base → base is empty
        let target = dir.path().join(".txt");
        fs::write(&target, "existing").unwrap();
        let result = safe_target(&target, true, None).unwrap();
        // Pure dotfile: suffix appended after the name → .txt-2
        assert_eq!(result, dir.path().join(".txt-2"));
    }

    #[test]
    fn test_safe_target_dotfile_multiple_collisions() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(".env"), "a").unwrap();
        fs::write(dir.path().join(".env-2"), "b").unwrap();
        fs::write(dir.path().join(".env-3"), "c").unwrap();
        let result = safe_target(&dir.path().join(".env"), true, None).unwrap();
        assert_eq!(result, dir.path().join(".env-4"));
    }

    #[test]
    fn test_rename_to_empty_name() {
        // Verify that rename_file handles the case where target has no filename.
        // The actual protection against empty slugs is in main.rs.
        // At the rename layer, we just verify it doesn't panic.
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("file.txt");
        fs::write(&source, "hello").unwrap();
        let target = dir.path().to_path_buf();

        // This should not panic regardless of OS behavior
        let _result = rename_file(&source, &target, true, false);
    }

    #[test]
    fn test_safe_target_overflow_protection() {
        let dir = tempfile::tempdir().unwrap();
        // Create 1000 collisions: file.txt, file-2.txt, ..., file-1001.txt
        fs::write(dir.path().join("file.txt"), "").unwrap();
        for i in 2..=1001u32 {
            fs::write(dir.path().join(format!("file-{i}.txt")), "").unwrap();
        }
        let result = safe_target(&dir.path().join("file.txt"), true, None);
        assert!(result.is_err(), "should error after 1000 collisions");
    }

    #[test]
    fn test_safe_target_just_under_cap() {
        let dir = tempfile::tempdir().unwrap();
        // Create 999 collisions: file.txt, file-2.txt, ..., file-1000.txt
        fs::write(dir.path().join("file.txt"), "").unwrap();
        for i in 2..=1000u32 {
            fs::write(dir.path().join(format!("file-{i}.txt")), "").unwrap();
        }
        let result = safe_target(&dir.path().join("file.txt"), true, None);
        assert!(result.is_ok(), "should succeed when slot 1001 is free");
        assert_eq!(result.unwrap(), dir.path().join("file-1001.txt"));
    }
}
