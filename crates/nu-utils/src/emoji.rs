/// Joins emoji into a single glyph (e.g. family or profession sequences).
const ZERO_WIDTH_JOINER: &str = "\u{200d}";

/// Requests the emoji (colored) presentation of the preceding character.
const VARIATION_SELECTOR_16: &str = "\u{fe0f}";

/// The five Fitzpatrick skin-tone modifiers, lightest to darkest.
const SKIN_TONES: [&str; 5] = [
    "\u{1f3fb}", // Light Skin Tone
    "\u{1f3fc}", // Medium-Light Skin Tone
    "\u{1f3fd}", // Medium Skin Tone
    "\u{1f3fe}", // Medium-Dark Skin Tone
    "\u{1f3ff}", // Dark Skin Tone
];

/// Whether `val` contains one of the code points that turn a run of characters into
/// a combined emoji, which needs special handling when measuring display width.
pub fn contains_emoji(val: &str) -> bool {
    val.contains(ZERO_WIDTH_JOINER)
        || val.contains(VARIATION_SELECTOR_16)
        || SKIN_TONES.iter().any(|skin_tone| val.contains(skin_tone))
}
