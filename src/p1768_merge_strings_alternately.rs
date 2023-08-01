#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word1_1: String = "abc".to_string();
        let word2_1: String = "pqr".to_string();
        assert_eq!(merge_alternately(word1_1, word2_1), "apbqcr".to_string());

        let word1_2: String = "ab".to_string();
        let word2_2: String = "pqrs".to_string();
        assert_eq!(merge_alternately(word1_2, word2_2), "apbqrs".to_string());

        let word1_3: String = "abcd".to_string();
        let word2_3: String = "pq".to_string();
        assert_eq!(merge_alternately(word1_3, word2_3), "apbqcd".to_string());
    }
}

pub fn merge_alternately(word1: String, word2: String) -> String {
    let word1: Vec<char> = word1.chars().collect();
    let word2: Vec<char> = word2.chars().collect();
    let mut ans: String = String::new();

    for i in 0..word1.len().max(word2.len()) {
        if i < word1.len() {
            ans.push(word1[i]);
        }
        if i < word2.len() {
            ans.push(word2[i]);
        }
    }

    ans
}
