#[test]
fn test() {
    let words1: Vec<String> = vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    assert_eq!(max_product(words1), 16);

    let words1: Vec<String> = vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    assert_eq!(max_product(words1), 4);

    let words1: Vec<String> = vec!["a", "aa", "aaa", "aaaa"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    assert_eq!(max_product(words1), 0);
}

// 题目的要求是两个word不能含有相同的字母
// 本题的关键在于如何比较两个word是否相同，常见的方法是哈希表，但是效率很低
// 效率更高的方法是位运算。将word的每一个字母映射成一个int类型的位中
pub fn max_product(words: Vec<String>) -> i32 {
    let mut mask: Vec<i32> = vec![0; words.len()];

    // 将每个word映射到一个int类型的数字mask
    // 构建mask这一步暂时不懂，但理解本题的基本逻辑
    for (i, word) in words.iter().enumerate() {
        let mut tmp: i32 = 0;
        for c in word.chars() {
            tmp |= 1 << (c as i32 - 'a' as i32);
        }
        mask[i] = tmp;
    }

    // 遍历word，如果两个word不含相同字母，更新答案
    let mut ans: usize = 0;
    for i in 0..words.len() {
        for j in i..words.len() {
            if mask[i] & mask[j] == 0 {
                ans = ans.max(words[i].len() * words[j].len());
            }
        }
    }

    ans as i32
}
