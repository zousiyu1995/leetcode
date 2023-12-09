#[test]
fn test() {
    assert_eq!(count_digits(7), 1);
    assert_eq!(count_digits(121), 2);
    assert_eq!(count_digits(1248), 4);
}

pub fn count_digits(num: i32) -> i32 {
    let mut tmp: i32 = num;
    let mut ans: i32 = 0;

    // 取出数字num的每一位数digit，如果num能被digit整除，更新答案
    while tmp > 0 {
        let digit: i32 = tmp % 10;
        if num % digit == 0 {
            ans += 1;
        }
        tmp /= 10;
    }

    ans
}
