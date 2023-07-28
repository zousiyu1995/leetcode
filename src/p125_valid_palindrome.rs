#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s1: String = String::from("A man, a plan, a canal: Panama");
        assert_eq!(is_palindrome(s1), true);

        let s2: String = String::from("race a car");
        assert_eq!(is_palindrome(s2), false);

        let s3: String = String::from("");
        assert_eq!(is_palindrome(s3), true);
    }
}

pub fn is_palindrome(s: String) -> bool {
    let s: String = s
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();

    let s_rev: String = s.chars().rev().collect();

    s == s_rev
}
