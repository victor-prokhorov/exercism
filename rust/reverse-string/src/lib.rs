#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

#[must_use]
#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

#[must_use]
#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
