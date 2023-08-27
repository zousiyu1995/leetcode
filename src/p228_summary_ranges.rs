#[test]
fn test() {
    assert_eq!(
        summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()]
    );
    assert_eq!(
        summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
        vec![
            "0".to_string(),
            "2->4".to_string(),
            "6".to_string(),
            "8->9".to_string()
        ]
    );
}

// 条件：无重复元素的有序数组
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut ans: Vec<String> = vec![];

    // [0, 1, 2, 4, 5, 7]
    //  ^     ^
    // low   high
    let mut low: usize = 0;
    for high in 0..nums.len() {
        // 如果到达数组边界或者不是连续递增
        if high + 1 == nums.len() || nums[high] != (nums[high + 1] - 1) {
            // low和high是两个值
            if low != high {
                ans.push(format!("{}->{}", nums[low], nums[high]));
            }
            // low和high一样
            else {
                ans.push(format!("{}", nums[low]));
            }
            low = high + 1;
        }
    }

    ans
}
