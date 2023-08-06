#[test]
fn test() {
    assert_eq!(remove_duplicates("abbaca".to_string()), "ca".to_string());
}

pub fn remove_duplicates(s: String) -> String {
    // 栈
    let mut stack: Vec<char> = vec![];

    for ch in s.chars() {
        if &ch == stack.last().unwrap_or(&' ') {
            stack.pop();
        } else {
            stack.push(ch);
        }
    }

    stack.iter().collect()
}
