#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(product_except_self(nums1), vec![24, 12, 8, 6]);

        let nums2: Vec<i32> = vec![-1, 1, 0, -3, 3];
        assert_eq!(product_except_self(nums2), vec![0, 0, 9, 0, 0]);
    }
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // 左乘积列表
    let mut l: Vec<i32> = vec![1; nums.len()];
    for i in 1..nums.len() {
        l[i] = l[i - 1] * nums[i - 1];
    }
    // 右成绩列表
    let mut r: Vec<i32> = vec![1; nums.len()];
    for i in (0..nums.len() - 1).rev() {
        r[i] = r[i + 1] * nums[i + 1];
    }

    let mut ans: Vec<i32> = Vec::new();
    for idx in 0..nums.len() {
        ans.push(l[idx] * r[idx]);
    }

    ans
}
