use std::collections::HashMap;

fn encode(msg: String, n: i32) -> Vec<i32> {
    let cypher: HashMap<char, i32> = ('a'..='z').zip(1..=26)
                                                .collect();
    let result: Vec<i32> = msg.chars()
        .zip(n.to_string()
            .chars()
            .cycle())
        .map(|(c, n)| cypher[&c] + n.to_digit(10)
            .unwrap() as i32)
        .collect();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fixed_tests() {
            assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
            assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
        }
    }
}
