#[test]
fn test() {
    assert_eq!(repeated_substring_pattern("abab".to_string()), true);
    assert_eq!(repeated_substring_pattern("aba".to_string()), false);
    assert_eq!(repeated_substring_pattern("abcabcabcabc".to_string()), true);
}

// 匹配可以用库函数，也可以用暴力匹配或KMP匹配等，懒，以后再写
pub fn repeated_substring_pattern(s: String) -> bool {
    // 构造字符串ss = s + s中，删除第一和倒数第一位的字符，在ss中寻找模式s即可
    let ss: String = format!("{}{}", &s[1..], &s[..s.len() - 1]);
    ss.contains(&s)
}
