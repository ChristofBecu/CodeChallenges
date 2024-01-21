use std::string::ToString;

fn printer_error(s: &str) -> String {
    let legal_chars: Vec<char> = ('a'..='m').collect();
    let error_count = s.chars().filter(|x| !legal_chars.contains(x)).count().to_string();
    let total_count = s.len().to_string();
    let result = format!("{}/{}", error_count, total_count);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_pass_all_the_tests_provided() {
        assert_eq!(&printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "3/56");
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "6/60");
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"), "11/65");
    }
}
