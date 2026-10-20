use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct MockAppearanceConfig {
    width_compact: u32,
    height_compact: u32,
    corner_radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct MockAppConfig {
    version: u32,
    appearance: MockAppearanceConfig,
}

#[test]
fn test_config_serde_roundtrip() {
    let cfg = MockAppConfig {
        version: 1,
        appearance: MockAppearanceConfig {
            width_compact: 200,
            height_compact: 36,
            corner_radius: 18.0,
        },
    };

    let serialized = toml::to_string(&cfg).expect("Serialization failed");
    let deserialized: MockAppConfig = toml::from_str(&serialized).expect("Deserialization failed");

    assert_eq!(cfg, deserialized);
}
