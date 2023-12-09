#[test]
fn test() {
    assert_eq!(
        decode_string("3[a]2[bc]".to_string()),
        "aaabcbc".to_string()
    );
    assert_eq!(
        decode_string("3[a2[c]]".to_string()),
        "accaccacc".to_string()
    );
    assert_eq!(
        decode_string("2[abc]3[cd]ef".to_string()),
        "abcabccdcdcdef".to_string()
    );
    assert_eq!(
        decode_string("abc3[cd]xyz".to_string()),
        "abccdcdcdxyz".to_string()
    );
}

// 栈。看到括号匹配就应该想到栈。
pub fn decode_string(s: String) -> String {
    let mut stack: Vec<(usize, String)> = vec![];
    let mut ans: String = String::new();
    let mut n: usize = 0;

    for c in s.chars() {
        match c {
            // 字母，收集起来
            'a'..='z' | 'A'..='Z' => ans.push(c),
            // 数字，收集起来
            '0'..='9' => {
                n = n * 10 + c.to_string().parse::<usize>().unwrap();
            }
            // [，入栈
            '[' => {
                stack.push((n, ans.clone()));
                ans.clear();
                n = 0;
            }
            // ]，出栈
            ']' => {
                if let Some(last) = stack.pop() {
                    ans = last.1 + ans.repeat(last.0).as_str();
                }
            }
            _ => unreachable!(),
        }
    }

    ans
}
