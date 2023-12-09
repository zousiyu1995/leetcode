#[test]
fn test() {
    let words1: Vec<String> = vec!["abba", "baba", "bbaa", "cd", "cd"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let ans1: Vec<String> = vec!["abba", "cd"].iter().map(|s| s.to_string()).collect();
    assert_eq!(remove_anagrams(words1), ans1);

    let words2: Vec<String> = vec!["a", "b", "c", "d", "e"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let ans2: Vec<String> = vec!["a", "b", "c", "d", "e"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(remove_anagrams(words2), ans2);
}

pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    // 栈
    let mut stack: Vec<String> = vec![];

    for word in words {
        // hashmap array
        let mut cnt: [i32; 26] = [0; 26];
        for ch in word.chars() {
            cnt[ch as usize - 'a' as usize] += 1;
        }
        for ch in stack.last().unwrap_or(&"".to_string()).chars() {
            cnt[ch as usize - 'a' as usize] -= 1;
        }
        // 用hashmap检查是否是字母异位词，如果不是字母异位词，入栈
        if cnt != [0; 26] {
            stack.push(word);
        }
    }

    stack
}
