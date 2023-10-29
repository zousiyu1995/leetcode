#[test]
fn test() {
    assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
}

// 方法1：排序，然后返回第n-k个元素即可。比较简单，不实现了

// 方法2：堆，但时间复杂度是O(NlogN)，本质上也是一种排序
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(nums);

    let mut ans: i32 = 0;
    for _ in 0..k {
        ans = heap.pop().unwrap();
    }

    ans
}

// 其他方法，比较复杂，以后再实现
