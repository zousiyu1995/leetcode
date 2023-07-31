#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let operations1: Vec<String> = ["--X", "X++", "X++"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(final_value_after_operations(operations1), 1);

        let operations2: Vec<String> = ["++X", "++X", "X++"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(final_value_after_operations(operations2), 3);

        let operations3: Vec<String> = ["X++", "++X", "--X", "X--"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(final_value_after_operations(operations3), 0);
    }
}

pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut x = 0;
    for s in operations {
        match s.as_str() {
            "X++" | "++X" => x += 1,
            "X--" | "--X" => x -= 1,
            _ => (),
        }
    }

    x
}
