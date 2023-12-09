#[test]
fn test() {
    assert_eq!(remove_stars("leet**cod*e".to_string()), "lecoe".to_string());
    assert_eq!(remove_stars("erase*****".to_string()), "".to_string());
}

pub fn remove_stars(s: String) -> String {
    // 需要考虑相邻元素 + 有消除操作 = 栈
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        if c == '*' {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    stack.iter().collect()
}
