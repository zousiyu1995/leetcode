#[test]
fn test() {
    assert_eq!(ways_to_buy_pens_pencils(20, 10, 5), 9);
    assert_eq!(ways_to_buy_pens_pencils(5, 10, 10), 1);
}

// 枚举
pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
    let total: i64 = total as i64;
    let cost1: i64 = cost1 as i64;
    let cost2: i64 = cost2 as i64;

    let mut ans: i64 = 0;
    // 枚举钢笔的购买方案，最多购买total / cost1只钢笔，一共1 + total / cost1种买法
    for i in 0..=(total / cost1) {
        // 剩下的钱够买铅笔，最多购买(total - cost1 * i) / cost2只铅笔，一共1 + (total - cost1 * i) / cost2种买法
        ans += 1 + (total - cost1 * i) / cost2;
    }

    ans
}
