///! **format_hex** formats an [u8] to an hexadecimal multiline format, for humain  readability.
///!
///! ```
///! let (l1, l2, l3) = FormatHex::new()
///!    .push_hex(b"ABC")
///!    .push_comment(" deb[")
///!    .push_hex(b"DEFGHIJ")
///!    .push_comment("]fin ")
///!    .push_hex(b"KLMNzZ?")
///!    .output();
///! assert_eq!(l1, "ABC deb[DEFGHIJ]fin KLMNzZ?");
///! assert_eq!(l2, "444.....4444444.....4444753");
///! assert_eq!(l3, "123.....456789A.....BCDEAAF");
///! ```
pub mod format_hex;
