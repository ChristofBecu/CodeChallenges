fn dead_ant_count(ants: &str) -> u32 {

    let total_a = ants.chars().filter(|&c| c == 'a').count() as u32;
    let total_n = ants.chars().filter(|&c| c == 'n').count() as u32;
    let total_t = ants.chars().filter(|&c| c == 't').count() as u32;

    let live_ants = ants.matches("ant").count() as u32;

    let dead_a = total_a.saturating_sub(live_ants);
    let dead_n = total_n.saturating_sub(live_ants);
    let dead_t = total_t.saturating_sub(live_ants);

    *[dead_a, dead_n, dead_t].iter().max().unwrap()
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
