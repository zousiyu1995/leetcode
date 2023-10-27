#[test]
fn test() {
    assert_eq!(get_sum_absolute_differences(vec![2, 3, 5]), [4, 3, 5]);
    assert_eq!(
        get_sum_absolute_differences(vec![1, 4, 6, 8, 10]),
        vec![24, 15, 13, 15, 21]
    );
}

// 本质上是数学题，用了前缀和技巧
// 设nums的索引从0到j，i是当前位置索引
// 注意：数组是有序的，因此可以不求绝对值。因为i左边的元素总是小于等于i，i右边的元素总是大于等于i。所以ans为，
// ans[i] = nums[i] - num[0] + ... + nums[i] - nums[i-1] +
//          nums[i] - nums[i] +
//          nums[i+1] - nums[i] + ... + nums[j] - nums[i]
// 将ans分为两部分，
// i左边的ans = i * nums[i] - presum[i]
// i右边的ans = -(n - i) * nums[i] + sufsum[i]
pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let n: usize = nums.len();
    let mut presum: Vec<i32> = vec![0; n + 1];
    // presum[i]是[0, i)内所有数的和
    for i in 0..n {
        presum[i + 1] = presum[i] + nums[i];
    }

    let mut ans: Vec<i32> = vec![];
    for i in 0..n {
        let left: i32 = (i as i32) * nums[i] - presum[i];
        let right: i32 = -1 * ((n - i) as i32) * nums[i] + (presum[n] - presum[i]);
        ans.push(left + right);
    }

    ans
}
