#[test]
fn test() {
    use method2::find_max_length;

    assert_eq!(find_max_length(vec![0, 1]), 2);
    assert_eq!(find_max_length(vec![0, 1, 0]), 2);
}

// 暴力法，时间复杂度O(N^2)，超时
mod method1 {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut presum: Vec<i32> = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            presum[i + 1] = presum[i] + nums[i];
        }

        // 满足题意的连续子数组的长度是和的两倍
        let mut max_len: i32 = 0;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if (j - i + 1) as i32 == 2 * (presum[j + 1] - presum[i]) {
                    max_len = max_len.max((j - i + 1) as i32);
                }
            }
        }

        max_len
    }
}

// 前缀和+哈希表，时间复杂度O(N)
mod method2 {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut presum: Vec<i32> = vec![0; nums.len() + 1];
        // 处理前缀和时，如果nums[i] == 0，当作-1来处理；nums[i] == 1，当作1来处理
        // 这样一来可以把问题转换成**如何求区间和为0的最长子数组**
        for i in 0..nums.len() {
            if nums[i] == 0 {
                presum[i + 1] = presum[i] + -1;
            } else {
                presum[i + 1] = presum[i] + 1;
            }
        }

        // 用哈希表储存前缀和和下标
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut max_len: usize = 0;

        for i in 0..presum.len() {
            // 如果出现了相同的前缀和，即得到了区间和为0的子数组，更新长度。反复更新长度就能找到最大值
            if map.contains_key(&presum[i]) {
                max_len = max_len.max(i - map.get(&presum[i]).unwrap());
            }
            // 如果没有出现相同的前缀和，更新哈希表
            else {
                map.insert(presum[i], i);
            }
        }

        max_len as i32
    }
}
