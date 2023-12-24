#[test]
fn test() {
    let s1: String = String::from("abcabcbb");
    assert_eq!(length_of_longest_substring(s1), 3);

    let s2: String = String::from("bbbbb");
    assert_eq!(length_of_longest_substring(s2), 1);

    let s3: String = String::from("pwwkew");
    assert_eq!(length_of_longest_substring(s3), 3);

    assert_eq!(length_of_longest_substring("".to_string()), 0);
}
// 维护窗口，只要窗口里有重复字符，移动窗口直至合法
// 用哈希集检查是否有重复元素
pub fn length_of_longest_substring(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();

    use std::collections::HashSet;
    let mut set: HashSet<char> = HashSet::new();
    let mut l: usize = 0;
    let mut ans: usize = 0;
    for r in 0..s.len() {
        while set.contains(&s[r]) {
            set.remove(&s[l]);
            l += 1;
        }
        set.insert(s[r]);
        ans = (r - l + 1).max(ans);
    }

    ans as i32
}
