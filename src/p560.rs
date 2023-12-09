#[test]
fn test() {
    use method2::subarray_sum;

    assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
}

// 时间复杂度O(N^2)，时间消耗太大
// 空间复杂度O(N)
mod method1 {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut presum: Vec<i32> = vec![0; nums.len() + 1];
        let mut ans: i32 = 0;

        // presum[i]是[0, i)内所有数的和
        for i in 0..nums.len() {
            presum[i + 1] = presum[i] + nums[i];
        }

        for i in 0..nums.len() {
            for j in i..nums.len() {
                if presum[j + 1] - presum[i] == k {
                    ans += 1;
                }
            }
        }

        ans
    }
}

// method1枚举了两个边界上的前缀和
// method2基于method1，用哈希表优化一下，只需要枚举右边界的前缀和
// 思路是，在查询到当前位置i的前缀和presum时，不用遍历之前位置的前缀和
// 而是去哈希表里找之前位置有多少前缀和是presum-k
// 因为presum - (presum - k) = k
// 和第1题两数之和有点类似
mod method2 {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new(); // key：前缀和；value：前缀和个数
        let mut presum: i32 = 0; // 该方法不需要前缀和数组
        let mut ans: i32 = 0;

        map.insert(0, 1);
        for num in nums {
            presum += num; // [0, i]的前缀和
            if let Some(&x) = map.get(&(presum - k)) {
                ans += x;
            }
            map.entry(presum).and_modify(|v| *v += 1).or_insert(1);
        }

        ans
    }
}
