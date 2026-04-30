//! Parser for projectMSDL .properties config files.
//!
//! The C++ SDL frontend uses a `key: value` format with dotted namespaces:
//! - `projectM.*` keys for visualizer settings
//! - `window.*` keys for window geometry
//! - `audio.*` keys for audio device selection
//!
//! This module parses these in two passes (window first, then projectM/audio)
//! and maps them into Settings.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Raw parsed properties from a .properties file, split by namespace.
pub struct ParsedProperties {
    pub window: HashMap<String, String>,
    pub projectm: HashMap<String, String>,
    pub audio: HashMap<String, String>,
}

/// Parse a .properties file into namespaced groups.
///
/// Format: `key: value` per line. Lines starting with `#` or `!` are comments.
/// Empty lines are ignored.
pub fn parse_properties_file(path: &Path) -> Result<ParsedProperties, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read properties file: {}", e))?;

    let mut window = HashMap::new();
    let mut projectm = HashMap::new();
    let mut audio = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
            continue;
        }

        // Split on first `: ` or `=`
        let (key, value) = if let Some(idx) = line.find(": ") {
            (&line[..idx], line[idx + 2..].trim())
        } else if let Some(idx) = line.find('=') {
            (&line[..idx], line[idx + 1..].trim())
        } else {
            continue; // Malformed line, skip
        };

        let key = key.trim();

        // Route to the appropriate namespace
        if let Some(suffix) = key.strip_prefix("window.") {
            window.insert(suffix.to_string(), value.to_string());
        } else if let Some(suffix) = key.strip_prefix("projectM.") {
            projectm.insert(suffix.to_string(), value.to_string());
        } else if let Some(suffix) = key.strip_prefix("audio.") {
            audio.insert(suffix.to_string(), value.to_string());
        }
    }

    Ok(ParsedProperties {
        window,
        projectm,
        audio,
    })
}

/// Parse a boolean value from a properties file.
/// Accepts "true"/"false" (case-insensitive).
fn parse_bool(value: &str) -> Option<bool> {
    match value.to_lowercase().as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

/// Convert parsed properties into Settings fields.
///
/// This is done in two passes as requested:
/// 1. Window pass: extract window geometry settings
/// 2. ProjectM/Audio pass: extract visualizer and audio settings
pub fn apply_properties(
    props: &ParsedProperties,
) -> PropertiesSettings {
    let mut settings = PropertiesSettings::default();

    // --- Pass 1: Window properties ---
    if let Some(v) = props.window.get("width") {
        settings.window_width = v.parse().ok();
    }
    if let Some(v) = props.window.get("height") {
        settings.window_height = v.parse().ok();
    }
    if let Some(v) = props.window.get("left") {
        settings.window_left = v.parse().ok();
    }
    if let Some(v) = props.window.get("top") {
        settings.window_top = v.parse().ok();
    }
    if let Some(v) = props.window.get("monitor") {
        settings.window_monitor = v.parse().ok();
    }
    if let Some(v) = props.window.get("overridePosition") {
        settings.window_override_position = parse_bool(v);
    }

    // --- Pass 2: ProjectM and Audio properties ---
    if let Some(v) = props.projectm.get("presetPath") {
        // Normalize Windows-style backslashes to forward slashes
        let normalized = v.replace("\\\\", "/").replace('\\', "/");
        // Remove trailing slash if present
        let normalized = normalized.trim_end_matches('/').to_string();
        settings.preset_path = Some(normalized.into());
    }
    if let Some(v) = props.projectm.get("shuffleEnabled") {
        settings.shuffle_enabled = parse_bool(v);
    }
    if let Some(v) = props.projectm.get("transitionDuration") {
        settings.transition_duration = v.parse().ok();
    }
    if let Some(v) = props.projectm.get("displayDuration") {
        settings.display_duration = v.parse().ok();
    }
    if let Some(v) = props.projectm.get("presetLocked") {
        settings.preset_locked = parse_bool(v);
    }
    if let Some(v) = props.projectm.get("enableSplash") {
        settings.enable_splash = parse_bool(v);
    }

    if let Some(v) = props.audio.get("device") {
        settings.audio_device = Some(v.to_string());
    }

    settings
}

/// All settings that can be extracted from a .properties file.
#[derive(Debug, Default)]
pub struct PropertiesSettings {
    // Window
    pub window_width: Option<u32>,
    pub window_height: Option<u32>,
    pub window_left: Option<i32>,
    pub window_top: Option<i32>,
    pub window_monitor: Option<u32>,
    pub window_override_position: Option<bool>,

    // ProjectM
    pub preset_path: Option<std::path::PathBuf>,
    pub shuffle_enabled: Option<bool>,
    pub transition_duration: Option<f64>,
    pub display_duration: Option<f64>,
    pub preset_locked: Option<bool>,
    pub enable_splash: Option<bool>,

    // Audio
    pub audio_device: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_parse_properties() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_projectm.properties");
        let mut f = fs::File::create(&path).unwrap();
        write!(
            f,
            r#"audio.device: BlackHole 2ch
projectM.displayDuration: 60.549
projectM.enableSplash: false
projectM.presetLocked: false
projectM.presetPath: /Users/delta/presets
projectM.shuffleEnabled: true
projectM.transitionDuration: 10
window.height: 448
window.left: 666
window.monitor: 2
window.overridePosition: false
window.top: 212
window.width: 850
"#
        )
        .unwrap();

        let props = parse_properties_file(&path).unwrap();
        let settings = apply_properties(&props);

        assert_eq!(settings.window_width, Some(850));
        assert_eq!(settings.window_height, Some(448));
        assert_eq!(settings.window_left, Some(666));
        assert_eq!(settings.window_top, Some(212));
        assert_eq!(settings.window_monitor, Some(2));
        assert_eq!(settings.window_override_position, Some(false));

        assert_eq!(
            settings.preset_path.as_ref().map(|p| p.to_str().unwrap()),
            Some("/Users/delta/presets")
        );
        assert_eq!(settings.shuffle_enabled, Some(true));
        assert_eq!(settings.transition_duration, Some(10.0));
        assert_eq!(settings.display_duration, Some(60.549));
        assert_eq!(settings.preset_locked, Some(false));
        assert_eq!(settings.enable_splash, Some(false));

        assert_eq!(settings.audio_device.as_deref(), Some("BlackHole 2ch"));

        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_windows_path_normalization() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_winpath.properties");
        let mut f = fs::File::create(&path).unwrap();
        write!(
            f,
            r#"projectM.presetPath: C:\\Users\\Dumbfuck\\presets\\"#
        )
        .unwrap();

        let props = parse_properties_file(&path).unwrap();
        let settings = apply_properties(&props);

        assert_eq!(
            settings.preset_path.as_ref().map(|p| p.to_str().unwrap()),
            Some("C:/Users/Dumbfuck/presets")
        );

        fs::remove_file(&path).ok();
    }
}
