#[test]
fn test() {
    let words1: Vec<String> = vec!["alice", "bob", "charlie"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let s1 = "abc".to_string();
    assert_eq!(is_acronym(words1, s1), true);
}

pub fn is_acronym(words: Vec<String>, s: String) -> bool {
    if words.len() != s.len() {
        return false;
    }

    for (word, ch) in words.iter().zip(s.chars()) {
        if word.chars().nth(0).unwrap() != ch {
            return false;
        }
    }

    true
}
