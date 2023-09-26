#[test]
fn test() {
    assert_eq!(pass_the_pillow(4, 5), 2);
    assert_eq!(pass_the_pillow(3, 2), 3);
    assert_eq!(pass_the_pillow(141, 506), 55);
}

// 数学题
// 每经过2*(n-1)的时间，枕头回到起点
pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let tmp = time % (2 * (n - 1));

    if tmp < n {
        tmp + 1
    } else {
        n - (tmp - (n - 1))
    }
}
