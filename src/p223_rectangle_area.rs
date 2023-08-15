#[test]
fn test() {
    assert_eq!(compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    assert_eq!(compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
}

pub fn compute_area(
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
) -> i32 {
    let area1: i32 = (ax2 - ax1) * (ay2 - ay1); // 矩形1的面积
    let area2: i32 = (bx2 - bx1) * (by2 - by1); // 矩形2的面积

    // x方向相交的长度
    let x: i32 = 0 // 两矩形不相交时，相交长度设为0
        .max(
            // 相交长度=两矩形左边界中最小的-两矩形右边界中最大的
            // 两矩形不相交时，该长度为负值，设为0
            ax2.min(bx2) - ax1.max(bx1),
        );
    // y方向相交的长度，与上面同理
    let y: i32 = 0.max(ay2.min(by2) - ay1.max(by1));

    area1 + area2 - (x * y)
}
