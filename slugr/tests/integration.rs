use std::fs;
use std::io::Write;
use std::process::Command;

fn slug_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_slugr"))
}

#[test]
fn test_dry_run_does_not_rename() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("My File.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(file.exists());
    assert!(!dir.path().join("my-file.txt").exists());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("my-file.txt"));
}

#[test]
fn test_execute_renames() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("My File.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(!file.exists());
    assert!(dir.path().join("my-file.txt").exists());
    assert_eq!(fs::read_to_string(dir.path().join("my-file.txt")).unwrap(), "hello");
}

#[test]
fn test_execute_verbose() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Loud File.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg("-v")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("loud-file.txt"));
}

#[test]
fn test_default_no_clobber_appends_suffix() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("my-file.txt"), "existing").unwrap();
    let file = dir.path().join("My File.txt");
    fs::write(&file, "new").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(dir.path().join("my-file.txt").exists());
    assert!(dir.path().join("my-file-2.txt").exists());
    assert_eq!(fs::read_to_string(dir.path().join("my-file-2.txt")).unwrap(), "new");
}

#[test]
fn test_snake_case_flag() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("My Cool File.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg("--snake")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(dir.path().join("my_cool_file.txt").exists());
}

#[test]
fn test_pascal_case_flag() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("my cool file.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg("--pascal")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(dir.path().join("MyCoolFile.txt").exists());
}

#[test]
fn test_recursive_renames() {
    let dir = tempfile::tempdir().unwrap();
    let sub = dir.path().join("Sub Dir");
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("Nested File.txt"), "nested").unwrap();
    fs::write(dir.path().join("Top File.txt"), "top").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg("-r")
        .arg(dir.path().to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(dir.path().join("sub-dir").join("nested-file.txt").exists());
    assert!(dir.path().join("top-file.txt").exists());
    assert!(dir.path().join("sub-dir").exists());
    assert!(!sub.exists());
}

#[test]
fn test_already_clean_skipped() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("already-clean.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.is_empty() || !stdout.contains("already-clean.txt"));
}

#[test]
fn test_stdin_pipe() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Piped File.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child
                .stdin
                .take()
                .unwrap()
                .write_all(format!("{}\n", file.display()).as_bytes())
                .unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    assert!(dir.path().join("piped-file.txt").exists());
}

// --- New integration tests ---

#[test]
#[cfg(target_os = "macos")]
fn test_case_only_rename() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("File.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    // Should produce file.txt, NOT file-2.txt
    let entries: Vec<String> = fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    assert!(entries.contains(&"file.txt".to_string()), "expected file.txt, got {entries:?}");
    assert!(!entries.contains(&"file-2.txt".to_string()), "should not produce file-2.txt");
}

#[test]
fn test_multiple_files_same_slug() {
    let dir = tempfile::tempdir().unwrap();
    let file1 = dir.path().join("My File.txt");
    let file2 = dir.path().join("My_File.txt");
    fs::write(&file1, "one").unwrap();
    fs::write(&file2, "two").unwrap();

    // Rename first file
    let output = slug_bin()
        .arg("-x")
        .arg(file1.to_str().unwrap())
        .output()
        .unwrap();
    assert!(output.status.success());

    // Rename second file — should get -2 suffix
    let output = slug_bin()
        .arg("-x")
        .arg(file2.to_str().unwrap())
        .output()
        .unwrap();
    assert!(output.status.success());

    assert!(dir.path().join("my-file.txt").exists());
    assert!(dir.path().join("my-file-2.txt").exists());
}

#[test]
fn test_unicode_transliteration_e2e() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Café Résumé.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(dir.path().join("cafe-resume.txt").exists());
}

#[test]
fn test_emoji_filename_e2e() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("🎉 Party.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    // Original should be gone, and some ASCII-named file should exist
    assert!(!file.exists());
    let entries: Vec<String> = fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    assert_eq!(entries.len(), 1);
    assert!(std::path::Path::new(&entries[0])
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("txt")));
    assert!(entries[0].is_ascii());
}

#[test]
fn test_keep_unicode_e2e() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Café.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg("--keep-unicode")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(dir.path().join("café.txt").exists());
}

#[test]
fn test_shell_metacharacters_safe() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("$(echo pwned).txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    // Should produce a safe filename without shell metacharacters
    let entries: Vec<String> = fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    assert_eq!(entries.len(), 1);
    assert!(!entries[0].contains('$'));
    assert!(!entries[0].contains('('));
}

#[test]
fn test_dotfile_preserved_e2e() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join(".gitignore");
    fs::write(&file, "*.o").unwrap();

    let output = slug_bin()
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    // Should be skipped (already clean)
    assert!(file.exists());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains(".gitignore") || stdout.is_empty());
}

#[test]
fn test_dotfile_with_extension_e2e() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join(".env.local");
    fs::write(&file, "SECRET=x").unwrap();

    let output = slug_bin()
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    // .env.local should be skipped (already clean with preserved dot)
    assert!(file.exists());
}

#[test]
fn test_exit_code_success() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Good File.txt");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0));
}

#[test]
#[cfg(unix)]
fn test_exit_code_on_error() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let sub = dir.path().join("readonly");
    fs::create_dir(&sub).unwrap();
    let file = sub.join("Bad File.txt");
    fs::write(&file, "hello").unwrap();
    fs::set_permissions(&sub, fs::Permissions::from_mode(0o555)).unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    // Restore permissions for cleanup
    fs::set_permissions(&sub, fs::Permissions::from_mode(0o755)).unwrap();

    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_nonexistent_file_handled() {
    let output = slug_bin()
        .arg("-x")
        .arg("/tmp/slug-test-definitely-nonexistent-file.txt")
        .output()
        .unwrap();

    // Should succeed (no error exit code) since nonexistent files are skipped in walk
    assert!(output.status.success());
}

#[test]
fn test_stdin_empty_lines_filtered() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Good File.txt");
    fs::write(&file, "hello").unwrap();

    let mut child = slug_bin()
        .arg("-x")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = child.stdin.as_mut().unwrap();
        // Send empty lines mixed with a valid path
        stdin.write_all(b"\n\n").unwrap();
        stdin.write_all(format!("{}\n", file.display()).as_bytes()).unwrap();
        stdin.write_all(b"\n").unwrap();
    }
    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert!(dir.path().join("good-file.txt").exists());
}

#[test]
fn test_recursive_empty_directory() {
    let dir = tempfile::tempdir().unwrap();
    let empty_sub = dir.path().join("Empty Dir");
    fs::create_dir(&empty_sub).unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg("-r")
        .arg(dir.path().to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    // "Empty Dir" should be renamed to "empty-dir"
    assert!(dir.path().join("empty-dir").exists());
    assert!(!empty_sub.exists());
}

#[test]
fn test_recursive_on_single_file() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Some File.txt");
    fs::write(&file, "hello").unwrap();

    // -r with a file argument should just rename the file
    let output = slug_bin()
        .arg("-x")
        .arg("-r")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(dir.path().join("some-file.txt").exists());
}

#[test]
fn test_snake_pascal_conflict_e2e() {
    let output = slug_bin()
        .arg("--snake")
        .arg("--pascal")
        .arg("file.txt")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("cannot be used with"), "expected conflict error, got: {stderr}");
}

#[test]
fn test_empty_slug_error() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("@@@");
    fs::write(&file, "hello").unwrap();

    let output = slug_bin()
        .arg("-x")
        .arg(file.to_str().unwrap())
        .output()
        .unwrap();

    // Should exit with error code 1
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("invalid"), "expected 'invalid' in error message, got: {stderr}");
    // Original file should still exist
    assert!(file.exists());
}

// --- Pipe mode integration tests ---

#[test]
fn test_pipe_mode_basic() {
    let output = slug_bin()
        .arg("--pipe")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"My Cool File.txt\n").unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "my-cool-file.txt");
}

#[test]
fn test_pipe_mode_raw() {
    let output = slug_bin()
        .arg("--pipe")
        .arg("--raw")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"My Blog Post Title!\n").unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "my-blog-post-title");
}

#[test]
fn test_pipe_mode_multiple_lines() {
    let output = slug_bin()
        .arg("--pipe")
        .arg("--raw")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"Cafe Resume\nHello World\n").unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines, vec!["cafe-resume", "hello-world"]);
}

#[test]
fn test_pipe_mode_snake_style() {
    let output = slug_bin()
        .arg("--pipe")
        .arg("--raw")
        .arg("--snake")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"My Blog Post\n").unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "my_blog_post");
}

#[test]
fn test_pipe_mode_pascal_style() {
    let output = slug_bin()
        .arg("--pipe")
        .arg("--raw")
        .arg("--pascal")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"my blog post\n").unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "MyBlogPost");
}

#[test]
fn test_pipe_mode_keep_unicode() {
    let output = slug_bin()
        .arg("--pipe")
        .arg("--raw")
        .arg("--keep-unicode")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all("Café Résumé\n".as_bytes()).unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "café-résumé");
}

#[test]
fn test_pipe_mode_filename_aware_default() {
    let output = slug_bin()
        .arg("--pipe")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"My Resume (Final).pdf\n").unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "my-resume-final.pdf");
}

#[test]
fn test_pipe_mode_raw_dots_not_preserved_as_ext() {
    let output = slug_bin()
        .arg("--pipe")
        .arg("--raw")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"my.blog.post\n").unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "my-blog-post");
}

#[test]
fn test_pipe_mode_empty_line_skipped() {
    let output = slug_bin()
        .arg("--pipe")
        .arg("--raw")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"\nHello World\n\n").unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines, vec!["hello-world"]);
}

#[test]
fn test_pipe_mode_empty_slug_warning() {
    let output = slug_bin()
        .arg("--pipe")
        .arg("--raw")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"@#$!\n").unwrap();
            child.wait_with_output()
        })
        .unwrap();

    assert!(output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("empty"), "expected warning about empty slug, got: {stderr}");
}
