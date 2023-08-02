#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1: Vec<i32> = vec![1, 12, -5, -6, 50, 3];
        assert_eq!(find_max_average(nums1, 4), 12.75);

        let nums2: Vec<i32> = vec![5];
        assert_eq!(find_max_average(nums2, 1), 5.00000);

        let nums3: Vec<i32> = vec![-1];
        assert_eq!(find_max_average(nums3, 1), -1.0);
    }
}

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    // 用第一个窗口的平均值作为初始值
    let mut sum: i32 = nums[0..(k as usize)].iter().sum::<i32>();
    let mut mean: f64 = sum as f64 / k as f64;
    // 移动窗口
    for i in (k as usize)..nums.len() {
        sum = sum + nums[i] - nums[i - (k as usize)];
        mean = mean.max(sum as f64 / k as f64);
    }

    mean
}
