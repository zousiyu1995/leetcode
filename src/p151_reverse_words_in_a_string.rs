#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1: String = String::from("the sky is blue");
        assert_eq!(reverse_words(s1), String::from("blue is sky the"));

        let s2: String = String::from("  hello world  ");
        assert_eq!(reverse_words(s2), String::from("world hello"));

        let s3: String = String::from("a good   example");
        assert_eq!(reverse_words(s3), String::from("example good a"));
    }
}

pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}
