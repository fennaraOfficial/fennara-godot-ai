const TOKEN_CHAR_APPROXIMATION: usize = 4;

pub(crate) fn estimate_text_tokens(text: &str) -> usize {
    text.len().div_ceil(TOKEN_CHAR_APPROXIMATION)
}
