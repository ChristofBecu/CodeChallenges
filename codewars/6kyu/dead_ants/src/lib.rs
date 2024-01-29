fn dead_ant_count(ants: &str) -> u32 {
    let counts = ['a', 'n', 't'].iter().map(|&c| ants.matches(c).count() as u32).collect::<Vec<_>>();
    let live_ants = ants.matches("ant").count() as u32;
    counts.iter().map(|&count| count.saturating_sub(live_ants)).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::dead_ant_count;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: u32) {
        assert_eq!(dead_ant_count(s), expected, "{ERR_MSG} with ants = \"{s}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("ant ant ant ant", 0);
        dotest("", 0);
        dotest(" ", 0);
        dotest("ant anantt aantnt", 2);
        dotest("ant ant .... a nt", 1);
    }
}
