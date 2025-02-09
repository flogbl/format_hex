//! **format_hex** formats an [u8] to an hexadecimal multiline format, for human readability.
//!
//! ```
//! let (l1, l2, l3) = FormatHex::new()
//!    .push_hex(b"ABC")
//!    .push_comment(" deb[")
//!    .push_hex(b"DEFGHIJ")
//!    .push_comment("]fin ")
//!    .push_hex(b"KLMNzZ?\n\r\0")
//!    .output();
//! assert_eq!(l1, "ABC deb[DEFGHIJ]fin KLMNzZ?↳←�");
//! assert_eq!(l2, "444.....4444444.....4444753000");
//! assert_eq!(l3, "123.....456789A.....BCDEAAFAD0");
//! ```
pub mod format_hex;
