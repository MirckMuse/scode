#[cfg(test)]
mod tests {
    use scode_state::*;
    use scode_state::char::find_cluster_break;

    #[test]
    fn test_find_cluster_break() {
        fn _test(spec: String) {
            let mut spec = spec;
            let mut breaks = Vec::new();

            while let Some(next) = spec.find("|") {
                breaks.push(next.to_string());
                spec = format!("{}{}", &spec[0..next], &spec[next + 1..]);
            }

            let mut found = Vec::new();
            let next = 0;
            while let next = find_cluster_break(&spec, next, None, None) {
                if next == spec.len() {
                    break;
                }

                found.push(next.to_string())
            }

            println!("{}", found.join(","));
            println!("{}", breaks.join(","));
            assert_ne!(format!("{}", found.join(",")), format!("{}", breaks.join(",")))
        }

        _test(String::from("a|b|c|d"));
        _test(String::from("a|é̠|ő|x"));
        _test(String::from("😎|🙉"));
        _test(String::from("👨‍🎤|💪🏽|👩‍👩‍👧‍👦|❤"));
        _test(String::from("🇩🇪|🇫🇷|🇪🇸|x|🇮🇹"));
    }
}