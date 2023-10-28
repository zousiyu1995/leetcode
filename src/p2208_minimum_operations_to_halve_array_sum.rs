#[test]
fn test() {
    assert_eq!(halve_array(vec![5, 19, 8, 1]), 3);
    assert_eq!(halve_array(vec![3, 8, 20]), 3);
}

// 本题可以用堆很快解决，但rust默认的二叉堆不能储存浮点数。麻烦在于给rust的二叉堆实现支持f64
// 在leetcode上看到有人把f64用结构体包装了一下，然后实现一些traits就能通过本题
// 浮点数存在计算精度问题，该解法不是最好的方法
#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct F64(f64);

impl Eq for F64 {}

impl Ord for F64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

pub fn halve_array(nums: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;
    let mut heap: BinaryHeap<F64> = BinaryHeap::new();

    // 构建二叉堆
    let mut init_sum: f64 = 0.0;
    for num in nums {
        init_sum += num as f64;
        heap.push(F64(num as f64));
    }

    // 每次挑最大数的减半，如果数组的和减少了一半，退出循环
    let mut sum: f64 = init_sum; // sum是数组新的和
    let mut count: i32 = 0;
    while sum > init_sum / 2.0 {
        count += 1;
        let max: f64 = heap.pop().unwrap().0;
        sum -= max / 2.0;
        heap.push(F64(max / 2.0));
    }

    count
}
