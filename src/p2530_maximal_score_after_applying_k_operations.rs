#[test]
fn test() {
    assert_eq!(max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
    assert_eq!(max_kelements(vec![1, 10, 3, 3, 3], 3), 17);
}

// 贪心、二叉堆
// 每次去最大值，操作k次
pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    use std::collections::BinaryHeap;

    let mut heap: BinaryHeap<i32> = BinaryHeap::from(nums); // 最大堆
    let mut ans: i64 = 0;

    for _ in 0..k {
        let max: i32 = heap.pop().unwrap();
        ans += max as i64;
        heap.push((max + 2) / 3);
    }

    ans
}
