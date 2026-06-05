pub fn estimate_tokens(bytes: u64) -> u64 {
    if bytes == 0 {
        0
    } else {
        bytes.div_ceil(4)
    }
}
