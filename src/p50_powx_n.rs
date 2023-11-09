#[test]
fn test() {
    use method1::my_pow;

    assert_eq!((my_pow(2.0, 10) - 1024.0).abs() < 1e-9, true);
    assert_eq!((my_pow(2.1, 3) - 9.261).abs() < 1e-9, true);
    assert_eq!((my_pow(2.0, -2) - 0.25).abs() < 1e-9, true);
}

// 分治
// 递归版本
mod method1 {
    pub fn quick_mul(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let y: f64 = quick_mul(x, n / 2);
        if n % 2 == 0 {
            y * y
        } else {
            y * y * x
        }
    }

    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n >= 0 {
            quick_mul(x, n)
        } else {
            1.0 / quick_mul(x, -n)
        }
    }
}
