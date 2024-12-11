/**
    \u{a0} is the Unicode escape sequence for non-breaking space (NBSP).

    It behaves similarly to a regular space but has different behavior when it comes to text wrapping.
    It prevents line breaks at that point, which is useful in certain formatting contexts
        (like keeping a name and a title together on the same line).
*/
pub fn clean_non_breaking_spaces(input: &str) -> String {
    input.replace("\u{a0}", " ")
}