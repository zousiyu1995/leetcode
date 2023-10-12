#[test]
fn test() {
    assert_eq!(find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
    assert_eq!(find_the_array_conc_val(vec![5, 14, 13, 8, 12]), 673);
}

// 双指针，模拟
pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    let mut left: usize = 0;
    let mut right: usize = nums.len() - 1;
    let mut ans: i64 = 0;

    while left < right {
        ans += format!("{}{}", nums[left], nums[right])
            .parse::<i64>()
            .unwrap();
        left += 1;
        right -= 1;
    }

    if left == right {
        ans += nums[left] as i64;
    }

    ans
}
