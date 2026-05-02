pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut hits: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            hits.push(line);
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

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
            search(query, contents)
        );
    }
}
