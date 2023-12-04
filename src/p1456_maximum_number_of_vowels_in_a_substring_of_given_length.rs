#[test]
fn test() {
    assert_eq!(max_vowels("abciiidef".to_string(), 3), 3);
    assert_eq!(max_vowels("aeiou".to_string(), 2), 2);
    assert_eq!(max_vowels("leetcode".to_string(), 3), 2);
    assert_eq!(
        max_vowels("ibpbhixfiouhdljnjfflpapptrxgcomvnb".to_string(), 33),
        7
    );
}

// 滑移窗口+哈希表
pub fn max_vowels(s: String, k: i32) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let n: usize = s.len();
    let map: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut cnt: i32 = 0;

    // 初始化第一个窗口的元音数量
    for i in 0..k as usize {
        if map.contains(&s[i]) {
            cnt += 1;
        }
    }
    // ans初始化为第一个窗口的元音数量
    let mut ans: i32 = cnt;

    // 移动窗口
    for i in 1..=(n - k as usize) {
        // 移除不在窗口的元音
        if map.contains(&s[i - 1]) {
            cnt -= 1;
        }
        // 添加新入窗口的元音
        if map.contains(&s[i + k as usize - 1]) {
            cnt += 1;
        }
        ans = ans.max(cnt);
    }

    ans
}
