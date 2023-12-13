pub fn make_smallest_palindrome(s: String) -> String {
    let mut s: Vec<char> = s.chars().collect();
    let mut l: usize = 0;
    let mut r: usize = s.len() - 1;

    while l < r {
        // 遇到不相等，两个字符都改成最小的那个
        if s[l] != s[r] {
            s[l] = s[l].min(s[r]);
            s[r] = s[l];
        }
        l += 1;
        r -= 1;
    }

    s.into_iter().collect()
}

#[test]
fn test() {
    assert_eq!(
        make_smallest_palindrome("egcfe".to_string()),
        "efcfe".to_string()
    );
    assert_eq!(
        make_smallest_palindrome("abcd".to_string()),
        "abba".to_string()
    );
    assert_eq!(
        make_smallest_palindrome("seven".to_string()),
        "neven".to_string()
    );
}
