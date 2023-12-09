#[test]
fn test() {
    assert_eq!(maximum_product(vec![0, 4], 5), 20);
    assert_eq!(maximum_product(vec![6, 3, 3, 2], 2), 216);
    assert_eq!(maximum_product(vec![24, 5, 64, 53, 26, 38], 54), 180820950);
}

// 小顶堆，每次给最小的元素加一，最终能得到最大乘积
pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    // 构建小顶堆
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for num in nums {
        heap.push(Reverse(num));
    }

    // 每次给最小的元素加一
    for _ in 0..k {
        let min: i32 = heap.pop().unwrap().0;
        heap.push(Reverse(min + 1));
    }

    const MOD: i64 = 1000000007;
    let mut ans: i64 = 1;
    for Reverse(num) in heap {
        ans = (ans * num as i64) % MOD;
    }

    ans as i32
}
