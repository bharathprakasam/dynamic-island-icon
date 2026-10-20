#[test]
fn test_config_defaults_validation() {
    let default_toml = r#"
        version = 1
        [appearance]
        width_compact = 200
        height_compact = 36
        width_expanded = 480
        height_expanded = 220
        corner_radius = 18.0
        opacity = 0.95
        theme = "default-dark"

        [behavior]
        autostart = true
        auto_hide_ms = 3000
        enable_animations = true
        animation_duration_ms = 250

        [[shortcuts]]
        key = "<Super>i"
        action = "toggle_island"
    "#;

    let parsed: toml::Value = toml::from_str(default_toml).expect("Valid TOML");
    assert_eq!(parsed.get("version").and_then(|v| v.as_integer()), Some(1));
    assert_eq!(
        parsed.get("appearance").and_then(|a| a.get("width_compact")).and_then(|w| w.as_integer()),
        Some(200)
    );
}
