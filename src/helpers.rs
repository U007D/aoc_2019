// Convert (newline-separated) string-encoded values into an `AsRef<Read>`er (`Vec<u8>`)
#[inline]
pub fn str_to_reader<S: AsRef<str>>(s: S) -> Vec<u8> {
    s.as_ref()
        .as_bytes()
        .iter()
        // `.to_vec()` is ugly here until `array` gains `IntoIterator`, but is ok for tests
        // see https://github.com/rust-lang/rust/pull/62959#issuecomment-514855137 et. al
        .flat_map(|n| n.to_ne_bytes().to_vec())
        .map(|ref_el| ref_el)
        .collect::<Vec<_>>()
}

// Convert a collection of (newline-separated) string-encoded values into a collection of
// `AsRef<Read>`ers (`Vec<Vec<u8>>`)
pub fn strs_to_readers<I: IntoIterator<Item = S>, S: AsRef<str>>(strs: I) -> Vec<Vec<u8>> {
    strs.into_iter().map(str_to_reader).collect::<Vec<Vec<_>>>()
}
