#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1: Vec<i32> = vec![1, 2];
        let nums2: Vec<i32> = vec![-2, -1];
        let nums3: Vec<i32> = vec![-1, 2];
        let nums4: Vec<i32> = vec![0, 2];
        assert_eq!(four_sum_count(nums1, nums2, nums3, nums4), 2);
    }
}

use std::collections::HashMap;

pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    // 把num1和num2中出现的数字之和插入hashmap
    for i in nums1.iter() {
        for j in nums2.iter() {
            map.entry(i + j)
                .and_modify(|x: &mut i32| *x += 1)
                .or_insert(1);
        }
    }

    // 在hashmap中找num3和num4中出现的数值之和的负值
    let mut ans: i32 = 0;
    for k in nums3.iter() {
        for l in nums4.iter() {
            if let Some(x) = map.get(&(-(k + l))) {
                ans += x; // x是满足i+j+k+l=0的组合的数目
            }
        }
    }

    ans
}
