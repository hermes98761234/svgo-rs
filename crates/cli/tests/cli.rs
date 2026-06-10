use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// A simple SVG fixture for testing.
const SIMPLE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <!-- a comment -->
  <g fill="red">
    <path d="M0 0h100v100H0z"/>
  </g>
</svg>
"#;

#[test]
fn optimize_file_in_place_smaller_and_parseable() {
    let tmp = TempDir::new().unwrap();
    let input_path = tmp.path().join("test.svg");
    fs::write(&input_path, SIMPLE_SVG).unwrap();

    let input_size = fs::metadata(&input_path).unwrap().len();

    Command::cargo_bin("svgo")
        .unwrap()
        .arg(input_path.to_str().unwrap())
        .assert()
        .success();

    let output = fs::read_to_string(&input_path).unwrap();
    let output_size = output.len();

    // Output should be smaller (comments removed, whitespace reduced)
    assert!(
        output_size < input_size as usize,
        "output ({output_size}) should be smaller than input ({input_size})"
    );

    // Output should be parseable SVG (starts with <svg or <?xml)
    assert!(
        output.starts_with("<") && output.contains("svg"),
        "output should be parseable SVG"
    );
}

#[test]
fn stdin_to_stdout_mode() {
    Command::cargo_bin("svgo")
        .unwrap()
        .write_stdin(SIMPLE_SVG)
        .assert()
        .success()
        .stdout(predicate::str::contains("svg"));
}

#[test]
fn show_plugins_lists_registered_names() {
    let output = Command::cargo_bin("svgo")
        .unwrap()
        .arg("--show-plugins")
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    // Should contain some known plugin names
    assert!(
        stdout.contains("removeDoctype"),
        "should list removeDoctype"
    );
    assert!(
        stdout.contains("removeComments"),
        "should list removeComments"
    );
    assert!(
        stdout.contains("convertColors"),
        "should list convertColors"
    );
}

#[test]
fn pretty_produces_indented_output() {
    let tmp = TempDir::new().unwrap();
    let input_path = tmp.path().join("test.svg");
    fs::write(&input_path, SIMPLE_SVG).unwrap();

    Command::cargo_bin("svgo")
        .unwrap()
        .arg("--pretty")
        .arg(input_path.to_str().unwrap())
        .assert()
        .success();

    // The file was optimized in-place, so check the file content
    let file_content = fs::read_to_string(&input_path).unwrap();
    // Pretty output should contain newlines and indentation
    assert!(
        file_content.contains('\n'),
        "pretty output should contain newlines"
    );
}

#[test]
fn output_flag_writes_to_file() {
    let tmp = TempDir::new().unwrap();
    let input_path = tmp.path().join("input.svg");
    let output_path = tmp.path().join("output.svg");
    fs::write(&input_path, SIMPLE_SVG).unwrap();

    Command::cargo_bin("svgo")
        .unwrap()
        .arg("-i")
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .assert()
        .success();

    assert!(output_path.exists(), "output file should exist");
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("svg"), "output should contain SVG");
}

#[test]
fn quiet_suppresses_output() {
    let tmp = TempDir::new().unwrap();
    let input_path = tmp.path().join("test.svg");
    fs::write(&input_path, SIMPLE_SVG).unwrap();

    Command::cargo_bin("svgo")
        .unwrap()
        .arg("--quiet")
        .arg(input_path.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}
