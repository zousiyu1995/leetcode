#[test]
fn test() {
    assert_eq!(sum_of_multiples(7), 21);
    assert_eq!(sum_of_multiples(10), 40);
    assert_eq!(sum_of_multiples(9), 30);
}

pub fn sum_of_multiples(n: i32) -> i32 {
    let mut ans: i32 = 0;

    for i in 1..=n {
        if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
            ans += i;
        }
    }

    ans
}
