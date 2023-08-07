#[test]
fn test() {
    assert_eq!(num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
    assert_eq!(num_subarray_product_less_than_k(vec![1, 2, 3], 0), 0);
}

// 双指针
pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    // k = 0 或 1 时，满足条件的子数字显然不存在
    if k <= 1 {
        return 0;
    }

    let mut prod: i32 = 1;
    let mut ans: usize = 0;
    let mut l: usize = 0;

    // 枚举右端点
    for (r, num) in nums.iter().enumerate() {
        prod *= num;
        // 缩小左端点
        while prod >= k {
            prod /= nums[l];
            l += 1;
        }
        // 此时，[l, r] 中的子数组均满足条件
        // 其中，以 r 为右端点的子数组个数为 r - l + 1
        ans += r - l + 1;
    }

    ans as i32
}
