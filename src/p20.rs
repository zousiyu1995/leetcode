#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s1: String = String::from("()");
        assert_eq!(is_valid(s1), true);

        let s2: String = String::from("()[]{}");
        assert_eq!(is_valid(s2), true);

        let s3: String = String::from("(]");
        assert_eq!(is_valid(s3), false);
    }
}

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            ']' | ')' | '}' if Some(c) != stack.pop() => return false,
            _ => (),
        }
    }

    stack.is_empty()
}
