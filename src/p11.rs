#[test]
fn test() {
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(max_area(vec![1, 1]), 1);
}

// 双指针，对撞
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    let (mut l, mut r): (usize, usize) = (0, height.len() - 1);

    // 从两端不断向内收缩 l 和 r 端点
    // 移动短板，面积可能增大
    // 移动长板，面积一定变小
    while l < r {
        let area: i32 = height[l].min(height[r]) * (r as i32 - l as i32);
        ans = ans.max(area);
        // 将短板向中间收缩
        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    ans
}
