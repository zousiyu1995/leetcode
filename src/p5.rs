#[test]
fn test() {
    assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
}

// 中心扩散法
pub fn longest_palindrome(s: String) -> String {
    let s: Vec<char> = s.chars().collect();
    let n: i32 = s.len() as i32;

    if n == 0 {
        return "".to_string();
    }

    let mut max_len: i32 = 0;
    let mut max_l: i32 = 0;
    for cur in 0..n {
        let mut l: i32 = cur - 1;
        let mut r: i32 = cur + 1;
        let mut len: i32 = 1;

        // 往左扩散
        while l >= 0 && s[l as usize] == s[cur as usize] {
            l -= 1;
            len += 1;
        }
        // 往右扩散
        while r < n && s[r as usize] == s[cur as usize] {
            r += 1;
            len += 1;
        }
        // 左右同时扩散
        while l >= 0 && r < n && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
            len += 2;
        }
        if len > max_len {
            max_len = len;
            max_l = l;
        }
    }

    (&s[(max_l + 1) as usize..(max_l + max_len + 1) as usize])
        .iter()
        .collect()
}
