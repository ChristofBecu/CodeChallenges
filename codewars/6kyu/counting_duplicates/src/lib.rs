use std::collections::HashSet;
fn count_duplicates(text: &str) -> u32 {
    let mut counted = HashSet::new();
    let mut duplicates = HashSet::new();

    for c in text.chars() {
        let c = c.to_ascii_lowercase();
        if !counted.insert(c) {
            duplicates.insert(c);
        }
    }

    duplicates.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}

