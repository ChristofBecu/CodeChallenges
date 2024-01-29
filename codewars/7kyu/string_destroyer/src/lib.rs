use std::collections::HashSet;

fn destroy(input_sets: Vec<HashSet<char>>) -> String {
    let alphabet: Vec<char> = ('a'..='z').collect();

    alphabet
        .iter()
        .map(|c| {
            input_sets
                .iter()
                .any(|set| set.contains(c))
                .then(|| '_')
                .unwrap_or(*c)
        })
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}


#[cfg(test)]
mod tests {
    use super::destroy;
    use std::collections::HashSet;

    #[test]
    fn basic_test1() {
        let mut input_set: Vec<HashSet<char>> = Vec::new();
        input_set.push(['A', 'b'].iter().cloned().collect());
        input_set.push(['C', 'd'].iter().cloned().collect());
        assert_eq!(
            destroy(input_set),
            "a _ c _ e f g h i j k l m n o p q r s t u v w x y z"
        );
    }

    #[test]
    fn basic_test2() {
        let mut input_set: Vec<HashSet<char>> = Vec::new();
        input_set.push(['B', 'b'].iter().cloned().collect());
        input_set.push(['C', 'm', 'f'].iter().cloned().collect());
        assert_eq!(
            destroy(input_set),
            "a _ c d e _ g h i j k l _ n o p q r s t u v w x y z"
        );
    }
}
