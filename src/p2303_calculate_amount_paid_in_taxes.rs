#[test]
fn test() {
    let ans1: f64 = (calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10) - 2.65).abs();
    assert!(ans1 <= 1e-5, "{}", ans1);

    let ans2: f64 = (calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2) - 0.25).abs();
    assert!(ans2 <= 1e-5);
}

// 模拟，关键是要搞清楚怎么算
pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
    let mut ans: i32 = 0;
    let mut lower: i32 = 0;

    for bracket in brackets {
        let upper: i32 = bracket[0];
        let percent: i32 = bracket[1];
        // 直接征税，大于0才是有效的税
        let tax: i32 = (income.min(upper) - lower) * percent;
        ans += tax.max(0);
        lower = upper;
    }

    ans as f64 / 100.0
}
