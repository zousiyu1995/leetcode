#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(min_partitions("32".to_string()), 3);
        assert_eq!(min_partitions("82734".to_string()), 8);
        assert_eq!(min_partitions("27346209830709182346".to_string()), 9);
    }
}

pub fn min_partitions(n: String) -> i32 {
    n.chars()
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .max()
        .unwrap()
}
