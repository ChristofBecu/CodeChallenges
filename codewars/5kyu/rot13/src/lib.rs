fn rot13(message: &str) -> String {
    message.chars().map(|c| if c.is_ascii_alphabetic() {
        let base: u8 = if c.is_ascii_lowercase() { b'a' } else { b'A' };
        let new_c: u8 = (c as u8 + 13 - base) % 26 + base;
        new_c as char
    }
    else {
        c
    }).collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::rot13;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("test", "grfg");
        dotest("Test", "Grfg");
        dotest("Hello, World!", "Uryyb, Jbeyq!");
        dotest("Uryyb, Jbeyq!", "Hello, World!");
        dotest("abcdefghijklmnopqrstuvwxyz", "nopqrstuvwxyzabcdefghijklm");
        dotest("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "NOPQRSTUVWXYZABCDEFGHIJKLM");
        dotest("1234567890", "1234567890");
        dotest("!@#$%^&*()", "!@#$%^&*()");
    }
}