#[test]
fn test() {
    assert_eq!(categorize_box(1000, 35, 700, 300), "Heavy".to_string());
}

// 简单的模拟题，没啥可说的
// 可以学习rust的bool值创建和模式匹配机制
pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
    let x: bool = length >= 10_i32.pow(4)
        || width >= 10_i32.pow(4)
        || height >= 10_i32.pow(4)
        || length as i64 * width as i64 * height as i64 >= 10_i64.pow(9);
    let y: bool = mass >= 100;

    match (x, y) {
        (true, true) => "Both".to_string(),
        (true, false) => "Bulky".to_string(),
        (false, true) => "Heavy".to_string(),
        (false, false) => "Neither".to_string(),
    }
}
