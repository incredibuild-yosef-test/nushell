use std::borrow::Cow;

/// Whether `string` holds any ASCII control character that stripping would remove.
///
/// LF (0x0A) is never counted. TAB (0x09) is counted only when `count_tab` is set:
/// the `&str` variant has always treated a lone tab as a reason to strip, the
/// `String` variant has not, and both keep that behavior. ESC (0x1B), the start of
/// every ANSI sequence, is always counted.
#[inline]
fn has_strippable_control(string: &str, count_tab: bool) -> bool {
    string
        .bytes()
        .any(|x| matches!(x, 0..=8 | 11..=31) || (count_tab && x == 9))
}

/// Removes ANSI escape codes and some ASCII control characters
///
/// Optimized for strings that rarely contain ANSI control chars.
/// Uses fast search to avoid reallocations.
///
/// Keeps `\n` removes `\r`, `\t` etc.
///
/// If parsing fails silently returns the input string
pub fn strip_ansi_unlikely(string: &str) -> Cow<'_, str> {
    if has_strippable_control(string, true)
        && let Ok(stripped) = String::from_utf8(strip_ansi_escapes::strip(string))
    {
        return Cow::Owned(stripped);
    }
    // Else case includes failures to parse!
    Cow::Borrowed(string)
}

/// Removes ANSI escape codes and some ASCII control characters
///
/// Optimized for strings that likely contain ANSI control chars.
///
/// Keeps `\n` removes `\r`, `\t` etc.
///
/// If parsing fails silently returns the input string
pub fn strip_ansi_likely(string: &str) -> Cow<'_, str> {
    // Check if any ascii control character except LF(0x0A = 10) is present,
    // which will be stripped. Includes the primary start of ANSI sequences ESC
    // (0x1B = decimal 27)
    if let Ok(stripped) = String::from_utf8(strip_ansi_escapes::strip(string)) {
        return Cow::Owned(stripped);
    }
    // Else case includes failures to parse!
    Cow::Borrowed(string)
}

/// Removes ANSI escape codes and some ASCII control characters
///
/// Optimized for strings that rarely contain ANSI control chars.
/// Uses fast search to avoid reallocations.
///
/// Keeps `\n` removes `\r`, `\t` etc.
///
/// If parsing fails silently returns the input string
pub fn strip_ansi_string_unlikely(string: String) -> String {
    if has_strippable_control(&string, false)
        && let Ok(stripped) = String::from_utf8(strip_ansi_escapes::strip(&string))
    {
        return stripped;
    }
    // Else case includes failures to parse!
    string
}

/// Removes ANSI escape codes and some ASCII control characters
///
/// Optimized for strings that likely contain ANSI control chars.
///
/// Keeps `\n` removes `\r`, `\t` etc.
///
/// If parsing fails silently returns the input string
pub fn strip_ansi_string_likely(string: String) -> String {
    // Check if any ascii control character except LF(0x0A = 10) is present,
    // which will be stripped. Includes the primary start of ANSI sequences ESC
    // (0x1B = decimal 27)
    if let Ok(stripped) = String::from_utf8(strip_ansi_escapes::strip(&string)) {
        return stripped;
    }
    // Else case includes failures to parse!
    string
}
