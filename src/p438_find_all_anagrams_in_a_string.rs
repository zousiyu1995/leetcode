#[test]
fn test() {
    assert_eq!(
        find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
        vec![0, 6]
    );
    assert_eq!(
        find_anagrams("abab".to_string(), "ab".to_string()),
        vec![0, 1, 2]
    );
}

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let m: usize = s.len();
    let n: usize = p.len();
    if n > m {
        return vec![];
    }

    // 准备p的哈希表
    let mut map_p: [i32; 26] = [0; 26];
    for c in p.chars() {
        map_p[c as usize - 'a' as usize] += 1;
    }

    let mut ans: Vec<i32> = vec![];
    let s: Vec<char> = s.chars().collect();

    // 维护一个大小是n的窗口，[start, end]
    for start in 0..(m - n + 1) {
        // 准备子串的哈希表
        let mut map_s: [i32; 26] = [0; 26];
        for end in start..(start + n) {
            map_s[s[end] as usize - 'a' as usize] += 1;
        }
        if map_p == map_s {
            ans.push(start as i32);
        }
    }

    ans
}
