pub mod search_utils {

    pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
        if ignore_case {
            let query = query.to_lowercase();
            contents
                .lines()
                .filter(|line| line.to_lowercase().contains(&query))
                .collect()
        } else {
            contents
                .lines()
                .filter(|line| line.contains(query))
                .collect()
        }
    }

    #[cfg(test)]
    mod tests {

        use super::search;
        #[test]
        fn one_result() {
            let query = "duct";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
not a duct-tape language.";

            assert_eq!(
                vec!["safe, fast, productive.", "not a duct-tape language."],
                search(query, contents, false)
            );
        }

        #[test]
        fn no_match() {
            let query = "duxct";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
not a duct-tape language.";

            let empty_vec: Vec<&str> = vec![];
            assert_eq!(empty_vec, search(query, contents, false));
        }

        #[test]
        fn case_sense() {
            let query = "HI";
            let contents = "\
hi.
say hi back, would you?";

            assert_ne!(
                vec!["hi", "say hi back, would you?"],
                search(query, contents, false)
            );
        }

        #[test]
        fn case_insenseitive() {
            let query = "RuSt";
            let contents = "\
Rust is a pretty cool language.
not to be confused with the song \"rust\" by BLS.
which is also pretty cool.		
		";
            assert_eq!(
                vec![
                    "Rust is a pretty cool language.",
                    "not to be confused with the song \"rust\" by BLS."
                ],
                search(query, contents, true)
            )
        }
    }
}
