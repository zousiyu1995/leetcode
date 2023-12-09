#[test]
fn test() {
    use method2::is_palindrome;

    // let s1: String = String::from("A man, a plan, a canal: Panama");
    // assert_eq!(is_palindrome(s1), true);

    // let s2: String = String::from("race a car");
    // assert_eq!(is_palindrome(s2), false);

    let s3: String = String::from("");
    assert_eq!(is_palindrome(s3), true);
}

// 比较取巧的方法是反转字符串，然后判断反转后的字符串是否等于原字符串
mod method1 {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .chars()
            .filter(|ch| ch.is_ascii_alphanumeric())
            .map(|ch| ch.to_ascii_lowercase())
            .collect();

        let s_rev: String = s.chars().rev().collect();

        s == s_rev
    }
}

// 经典的方法自然是双指针
mod method2 {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .chars()
            .filter(|ch| ch.is_ascii_alphanumeric())
            .map(|ch| ch.to_ascii_lowercase())
            .collect();

        // 特殊处理一下空字符串
        if s == "".to_string() {
            return true;
        }

        let mut l: usize = 0;
        let mut r: usize = s.len() - 1;

        while l < r {
            if &s[l..l + 1] != &s[r..r + 1] {
                return false;
            }
            l += 1;
            r -= 1;
        }

        true
    }
}
