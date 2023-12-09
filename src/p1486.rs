#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(xor_operation(5, 0), 8);
        assert_eq!(xor_operation(4, 3), 8);
        assert_eq!(xor_operation(1, 7), 7);
        assert_eq!(xor_operation(10, 5), 2);
    }
}

pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut ans: i32 = 0;
    for i in 0..n {
        ans ^= start + 2 * i;
    }
    ans
}
