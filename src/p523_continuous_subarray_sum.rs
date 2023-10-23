#[test]
fn test() {
    use method2::check_subarray_sum;

    assert_eq!(check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    assert_eq!(check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
    assert_eq!(check_subarray_sum(vec![23, 2, 6, 4, 7], 13), false);
    assert_eq!(check_subarray_sum(vec![23, 2, 4, 6, 6], 7), true);
    assert_eq!(check_subarray_sum(vec![5, 0, 0, 0], 3), true);
}

// 前缀和+滑移窗口
// 时间复杂度O(N^2)，会超时
mod method1 {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut presum: Vec<i32> = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            presum[i + 1] = presum[i] + nums[i];
        }

        // 用一个大小>=2的窗口检查presum，获取子数组的和，然后检查该和是否是k的倍数
        // sum([i..j]) = presum[j + 1] - presum[i]
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if (presum[j + 1] - presum[i]) % k == 0 {
                    return true;
                }
            }
        }

        false
    }
}

// method1的暴力法超时，此题实际上是个数学题，运用了同余定理降低时间复杂度
// 前缀和+哈希表
mod method2 {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut presum: Vec<i32> = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            presum[i + 1] = presum[i] + nums[i];
        }

        // sum([i..j]) = presum[j + 1] - presum[i] = n * k
        // 于是有，presum[j + 1] / k - presum[i] / k = n
        // 根据同余定理有，presum[j + 1] % k = presum[i] % k
        // 所以，如果两个前缀和除以k的余数相同，且前缀和之间下标大于等于2，则满足题意
        // 遍历前缀和数组，用哈希表储存余数和下标即可
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new();

        for i in 0..presum.len() {
            let rem: i32 = presum[i] % k;
            // 如果哈希表中有相同的余数，不要更新哈希表
            if map.contains_key(&rem) {
                // 如果数组长度大于等于2，满足题意
                if i - map[&rem] >= 2 {
                    return true;
                }
            }
            // 否则，更新哈希表
            else {
                map.insert(rem, i);
            }
        }

        false
    }
}
