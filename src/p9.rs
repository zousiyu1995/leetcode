#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
    }
}

// pub fn is_palindrome(x: i32) -> bool {
//     // 转换为字符串，逆转，比较
//     x.to_string().chars().rev().eq(x.to_string().chars())
// }

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    // 逆转数字
    let mut x_copy: i32 = x;
    let mut x_rev: i32 = 0;
    while x_copy != 0 {
        x_rev = x_rev * 10 + x_copy % 10;
        x_copy /= 10;
    }

    x == x_rev
}
