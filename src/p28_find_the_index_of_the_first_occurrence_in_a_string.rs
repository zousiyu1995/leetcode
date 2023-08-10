#[test]
fn test() {
    use method2::str_str;

    let h1: String = "sadbutsad".to_string();
    let n1: String = "sad".to_string();
    assert_eq!(str_str(h1, n1), 0);

    let h2: String = "leetcode".to_string();
    let n2: String = "leeto".to_string();
    assert_eq!(str_str(h2, n2), -1);
}

// 库函数
mod method1 {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(v) => v as i32,
            None => -1,
        }
    }
}

// 暴力法
mod method2 {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let txt: Vec<char> = haystack.chars().collect();
        let pattern: Vec<char> = needle.chars().collect();

        if txt.len() < pattern.len() {
            return -1;
        }

        // txt     = ['h', 'e', 'l', 'l', 'o']
        // pattern = ['h', 'e']
        for i in 0..=(txt.len() - pattern.len()) {
            for j in 0..pattern.len() {
                // 字符不匹配，移动 i，即移动 pattern
                if txt[i + j] != pattern[j] {
                    break;
                }
                // 字符匹配，移动 j，继续匹配下一位字符
                // 匹配完了，返回 i
                if j == pattern.len() - 1 {
                    return i as i32;
                }
            }
        }

        -1
    }
}

// TODO: KMP
mod method3 {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        todo!()
    }
}
