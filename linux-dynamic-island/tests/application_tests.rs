#[test]
fn test_desktop_entry_parsing() {
    let desktop_file_content = r#"
[Desktop Entry]
Version=1.0
Type=Application
Name=Dynamic Browser
Comment=Fast and private browser
Exec=dynamic-browser --profile %u
Icon=web-browser
Terminal=false
Categories=Network;WebBrowser;
NoDisplay=false
"#;

    // Direct string parsing logic validation
    let lines: Vec<&str> = desktop_file_content.lines().collect();
    assert!(lines.iter().any(|l| l.starts_with("Name=Dynamic Browser")));
    assert!(lines.iter().any(|l| l.starts_with("Exec=")));
}

#[test]
fn test_clean_exec_command() {
    let raw_exec = "firefox %u --new-window %F";
    let cleaned = raw_exec
        .split_whitespace()
        .filter(|arg| !arg.starts_with('%'))
        .collect::<Vec<_>>()
        .join(" ");

    assert_eq!(cleaned, "firefox --new-window");
}
