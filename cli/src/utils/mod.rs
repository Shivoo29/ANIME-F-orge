pub mod progress;

use std::path::Path;

/// Format file size in human-readable format
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Get file extension
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
}

/// Validate if string is a valid identifier
pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let first_char = s.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }

    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500.00 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1536 * 1024), "1.50 MB");
    }

    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("valid_name"));
        assert!(is_valid_identifier("_private"));
        assert!(is_valid_identifier("MyClass"));
        assert!(!is_valid_identifier("123invalid"));
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("with-dash"));
    }
}
