#[test]
fn test() {
    assert_eq!(subtract_product_and_sum(234), 15);
    assert_eq!(subtract_product_and_sum(4421), 21);
}

pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut n: i32 = n;
    let mut sum: i32 = 0;
    let mut prod: i32 = 1;

    while n != 0 {
        let x: i32 = n % 10;
        n = n / 10;
        sum += &x;
        prod *= &x;
    }

    prod - sum
}
