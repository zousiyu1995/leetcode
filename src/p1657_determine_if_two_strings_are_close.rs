#[test]
fn test() {
    let word1_1: String = "abc".to_string();
    let word2_1: String = "bca".to_string();
    assert_eq!(close_strings(word1_1, word2_1), true);
}

pub fn close_strings(word1: String, word2: String) -> bool {
    use std::collections::HashMap;

    let mut map1: HashMap<char, i32> = HashMap::new();
    word1.chars().for_each(|c| *map1.entry(c).or_insert(0) += 1);
    let mut map2: HashMap<char, i32> = HashMap::new();
    word2.chars().for_each(|c| *map2.entry(c).or_insert(0) += 1);

    // 如果字符一样，出现次数也一样，两字符相似
    let mut k1: Vec<&char> = map1.keys().collect();
    k1.sort();
    let mut k2: Vec<&char> = map2.keys().collect();
    k2.sort();
    let mut v1: Vec<&i32> = map1.values().collect();
    v1.sort();
    let mut v2: Vec<&i32> = map2.values().collect();
    v2.sort();

    k1 == k2 && v1 == v2
}
