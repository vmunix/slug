use std::path::PathBuf;
use walkdir::WalkDir;

/// Collect all file and directory paths under `root`, bottom-up.
/// Bottom-up ensures children are renamed before parents.
/// If `recursive` is false, only collects the given paths directly.
#[must_use]
pub fn collect_paths(paths: &[PathBuf], recursive: bool) -> Vec<PathBuf> {
    let mut result = Vec::new();

    for path in paths {
        if !path.exists() {
            eprintln!("slugr: warning: '{}': not found", path.display());
            continue;
        }

        if !recursive || path.is_file() {
            result.push(path.clone());
            continue;
        }

        // Recursive directory traversal, bottom-up (contents_first)
        for entry in WalkDir::new(path).contents_first(true) {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("slugr: warning: {e}");
                    continue;
                }
            };
            // Skip the root directory itself
            if entry.path() == path {
                continue;
            }
            result.push(entry.path().to_path_buf());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_collect_single_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "hello").unwrap();

        let result = collect_paths(std::slice::from_ref(&file), false);
        assert_eq!(result, vec![file]);
    }

    #[test]
    fn test_collect_non_recursive() {
        let dir = tempfile::tempdir().unwrap();
        let file_a = dir.path().join("A File.txt");
        let file_b = dir.path().join("B File.txt");
        fs::write(&file_a, "a").unwrap();
        fs::write(&file_b, "b").unwrap();

        let result = collect_paths(&[file_a.clone(), file_b.clone()], false);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&file_a));
        assert!(result.contains(&file_b));
    }

    #[test]
    fn test_collect_recursive_bottom_up() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("Sub Dir");
        fs::create_dir(&sub).unwrap();
        let file = sub.join("Nested File.txt");
        fs::write(&file, "nested").unwrap();

        let result = collect_paths(&[dir.path().to_path_buf()], true);

        // File should come before directory (bottom-up)
        let file_pos = result.iter().position(|p| p == &file).unwrap();
        let dir_pos = result.iter().position(|p| p == &sub).unwrap();
        assert!(file_pos < dir_pos, "files must come before their parent dirs");
    }

    #[test]
    fn test_collect_recursive_skips_root() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "hello").unwrap();

        let result = collect_paths(&[dir.path().to_path_buf()], true);
        // Should not include the root directory itself
        assert!(!result.contains(&dir.path().to_path_buf()));
    }

    #[test]
    fn test_collect_non_existent_path() {
        let result = collect_paths(&[PathBuf::from("/tmp/definitely-does-not-exist-slug-test")], false);
        assert!(result.is_empty());
    }

    #[test]
    fn test_collect_empty_directory_recursive() {
        let dir = tempfile::tempdir().unwrap();
        let result = collect_paths(&[dir.path().to_path_buf()], true);
        assert!(result.is_empty());
    }

    #[test]
    #[cfg(unix)]
    fn test_collect_symlink_in_tree() {
        let dir = tempfile::tempdir().unwrap();
        let real_file = dir.path().join("real.txt");
        fs::write(&real_file, "hello").unwrap();
        let link = dir.path().join("link.txt");
        std::os::unix::fs::symlink(&real_file, &link).unwrap();

        let result = collect_paths(&[dir.path().to_path_buf()], true);
        // Symlink should appear in results
        assert!(result.contains(&link));
        assert!(result.contains(&real_file));
    }

    #[test]
    fn test_collect_deeply_nested() {
        let dir = tempfile::tempdir().unwrap();
        let mut current = dir.path().to_path_buf();
        for i in 1..=5 {
            current = current.join(format!("level{i}"));
            fs::create_dir(&current).unwrap();
        }
        let deep_file = current.join("deep.txt");
        fs::write(&deep_file, "deep").unwrap();

        let result = collect_paths(&[dir.path().to_path_buf()], true);
        assert!(result.contains(&deep_file), "deep file should be collected");
        // File should come before all its parent dirs (bottom-up)
        let file_pos = result.iter().position(|p| p == &deep_file).unwrap();
        for ancestor in deep_file.ancestors().skip(1) {
            if ancestor == dir.path() {
                break;
            }
            if let Some(dir_pos) = result.iter().position(|p| p == ancestor) {
                assert!(file_pos < dir_pos, "file must come before ancestor dirs");
            }
        }
    }

    #[test]
    fn test_collect_mixed_files_and_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("subdir");
        fs::create_dir(&sub).unwrap();
        let file_a = dir.path().join("a.txt");
        let file_b = sub.join("b.txt");
        fs::write(&file_a, "a").unwrap();
        fs::write(&file_b, "b").unwrap();

        let result = collect_paths(&[dir.path().to_path_buf()], true);
        assert!(result.contains(&file_a));
        assert!(result.contains(&file_b));
        assert!(result.contains(&sub));
        // Subdir contents before subdir itself
        let b_pos = result.iter().position(|p| p == &file_b).unwrap();
        let sub_pos = result.iter().position(|p| p == &sub).unwrap();
        assert!(b_pos < sub_pos, "files in subdir must come before subdir");
    }
}
