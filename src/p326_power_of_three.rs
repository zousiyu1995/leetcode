#[test]
fn test() {
    assert_eq!(is_power_of_three(27), true);
    assert_eq!(is_power_of_three(0), false);
    assert_eq!(is_power_of_three(9), true);
    assert_eq!(is_power_of_three(45), false);
}

// 数学，考察对循环的理解
pub fn is_power_of_three(n: i32) -> bool {
    let mut n: i32 = n;

    if n <= 0 {
        return false;
    }

    while n % 3 == 0 {
        n = n / 3;
    }

    n == 1
}
