// function that removes first and last character of a str
// used for unquoting etc.
// returns a tuple (surrounding character, unsurrounded string)
// does not check if length is sufficient, this check is handled by grammar
pub fn unsurround(input: &str) -> (char, String) {
    (
        input.chars().next().unwrap(),
        input.chars()
            .skip(1)
            .take(input.len() - 2)
            .collect(),
    )
}
