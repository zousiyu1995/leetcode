#[test]
fn test() {
    assert_eq!(is_power_of_three(27), true);
    assert_eq!(is_power_of_three(0), false);
    assert_eq!(is_power_of_three(9), true);
    assert_eq!(is_power_of_three(45), false);
}

// 数学，理解循环
pub fn is_power_of_three(n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    let mut n: i32 = n;
    while n % 3 == 0 {
        n = n / 3;
    }

    n == 1
}
