#[test]
fn test_empty_search() {
    let expected = std::fs::read_to_string("tests/test.txt")
        .expect("Failed to read test.txt");

    let expected_normalized = normalize_text(&expected);

    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("") // Empty string as regex argumeng
        .arg("tests/test.txt")
        .output()
        .expect("failed to execute process");

    let output_normalized = normalize_text(&String::from_utf8_lossy(&output.stdout));

    assert_eq!(output_normalized, expected_normalized);
    assert!(output.status.success(), "The command failed to execute successfully");
}

// Function to normalize text (removes BOM and normalize line endings)
// for better comparisons
fn normalize_text(text: &str) -> String {
    // Remove BOM
    let text = if text.starts_with('\u{feff}') {
        &text[3..] // Skip BOM
    } else {
        text
    };

    // Normalize line endings: Replace CRLF with LF
    text.replace("\r\n", "\n").to_string()
}
