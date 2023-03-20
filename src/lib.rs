use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(feature = "simd-accel"))] {
        fn is_str_latin1_impl(buffer: &str) -> Option<usize> {
            let mut bytes = buffer.as_bytes();
            let mut total = 0;
            loop {
                if let Some((byte, offset)) = validate_ascii(bytes) {
                    total += offset;
                    if byte > 0xC3 {
                        return Some(total);
                    }
                    bytes = &bytes[offset + 2..];
                    total += 2;
                } else {
                    return None;
                }
            }
        }
    }
}
