#[test]
fn test() {
    assert_eq!(is_ugly(6), true);
    assert_eq!(is_ugly(1), true);
    assert_eq!(is_ugly(14), false);
}

// 数学，考察对循环的理解
pub fn is_ugly(n: i32) -> bool {
    let mut n = n;

    if n <= 0 {
        return false;
    }

    // 反复除以2、3、5
    let factors: Vec<i32> = vec![2, 3, 5];
    for factor in factors {
        while n % factor == 0 {
            n /= factor;
        }
    }

    n == 1
}
