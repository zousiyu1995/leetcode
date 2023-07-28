#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p1: String = "abba".to_string();
        let s1: String = "dog cat cat dog".to_string();
        assert_eq!(word_pattern(p1, s1), true);

        let p2: String = "abba".to_string();
        let s2: String = "dog cat cat fish".to_string();
        assert_eq!(word_pattern(p2, s2), false);

        let p3: String = "aaaa".to_string();
        let s3: String = "dog cat cat dog".to_string();
        assert_eq!(word_pattern(p3, s3), false);
    }
}
use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let s: Vec<&str> = s.split_whitespace().collect();

    // pattern和s长度不一样，不满足
    if pattern.len() != s.len() {
        return false;
    }

    let mut pattern2s: HashMap<char, &str> = HashMap::new();
    let mut s2pattern: HashMap<&str, char> = HashMap::new();
    for (ch, word) in pattern.iter().zip(s) {
        // 如果在hashmap里，但是值不能对应，不满足
        if pattern2s.contains_key(ch) && pattern2s.get(ch).unwrap() != &word {
            return false;
        } else if s2pattern.contains_key(word) && s2pattern.get(word).unwrap() != ch {
            return false;
        }
        pattern2s.insert(*ch, word);
        s2pattern.insert(word, *ch);
    }

    true
}
