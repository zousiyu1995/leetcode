#[test]
fn test() {
    assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    assert_eq!(longest_valid_parentheses("".to_string()), 0);
}

// 栈
pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut stack: Vec<i32> = vec![];
    stack.push(-1);
    let mut ans: i32 = 0;

    for (idx, ch) in s.chars().enumerate() {
        // (，入栈
        if ch == '(' {
            stack.push(idx as i32);
        }
        // )，出栈
        else {
            stack.pop();
            // 栈为空，说明是最后一个没有被匹配的右括号
            if stack.is_empty() {
                stack.push(idx as i32);
            }
            // 栈不为空，可以获得当前有效括号长度
            else {
                ans = ans.max(idx as i32 - stack.last().unwrap());
            }
        }
    }

    ans
}
