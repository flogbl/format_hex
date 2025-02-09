#[derive(Debug)]
pub struct FormatHex {
    char_l: String,
    hex_l1: String,
    hex_l2: String,
}
impl FormatHex {
    ///
    /// New instance
    pub fn new() -> Self {
        Self {
            char_l: String::new(),
            hex_l1: String::new(),
            hex_l2: String::new(),
        }
    }
    ///
    /// Push an [u8] array
    pub fn push_hex(&mut self, arr: &[u8]) -> &mut Self {
        for el in arr {
            let chr = *el as char;
            if chr.is_alphanumeric() || chr.is_ascii_punctuation() || chr == ' ' {
                self.char_l.push(chr);
            } else if chr == '\n' {
                self.char_l.push('\u{21B3}');
            } else if chr == '\r' {
                self.char_l.push('\u{2190}');
            } else {
                self.char_l.push('\u{FFFD}');
            }

            let (high, low) = byte2hex(*el);
            self.hex_l1.push(high);
            self.hex_l2.push(low);
        }
        self
    }
    ///
    /// Push comment (display points in the hexadecimal part)
    pub fn push_comment(&mut self, txt: &str) -> &mut Self {
        self.char_l.push_str(txt);
        let fill = ".".repeat(txt.len());
        self.hex_l1.push_str(&fill);
        self.hex_l2.push_str(&fill);
        self
    }
    ///
    /// Output it
    pub fn output(&mut self) -> (String, String, String) {
        (
            self.char_l.to_string(),
            self.hex_l1.to_string(),
            self.hex_l2.to_string(),
        )
    }
}
///
/// Private : convert to hexadecimal
fn byte2hex(byte: u8) -> (char, char) {
    let table = b"0123456789ABCDEF";
    let high = table[((byte & 0xf0) >> 4) as usize];
    let low = table[(byte & 0x0f) as usize];
    (high as char, low as char)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        let (l1, l2, l3) = FormatHex::new()
            .push_hex(b"ABC")
            .push_comment(" deb[")
            .push_hex(b"DEFGHIJ")
            .push_comment("]fin ")
            .push_hex(b"KLMNzZ?")
            .output();
        assert_eq!(l1, "ABC deb[DEFGHIJ]fin KLMNzZ?");
        assert_eq!(l2, "444.....4444444.....4444753");
        assert_eq!(l3, "123.....456789A.....BCDEAAF");
    }

    #[test]
    fn special_chars() {
        let (l1, l2, l3) = FormatHex::new().push_hex(b"\n\r\0").output();
        assert_eq!(l1, "↳←�");
        assert_eq!(l2, "000");
        assert_eq!(l3, "AD0");
    }
}
