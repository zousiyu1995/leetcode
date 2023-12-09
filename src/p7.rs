#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(0), 0);
    }
}

pub fn reverse(x: i32) -> i32 {
    let mut x: i32 = x;
    let mut ans: i32 = 0;

    while x != 0 {
        // 如果超过i32的上下限
        if ans > i32::MAX / 10 || ans < i32::MIN / 10 {
            return 0;
        }
        // 取余能得到最后一位数字
        let rem: i32 = x % 10;
        x = x / 10;
        ans = ans * 10 + rem;
    }

    ans
}
