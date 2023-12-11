// 找一个子数组，最多只包含一个0
// 维护窗口，要求窗口的长度 <= 1的数量 + 1
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut ans: usize = 0;
    let mut l: usize = 0;
    let mut cnt: usize = 0;
    for r in 0..nums.len() {
        if nums[r] == 1 {
            cnt += 1;
        }
        if (r - l + 1) > cnt + 1 {
            if nums[l] == 1 {
                cnt -= 1;
            }
            l += 1;
        }

        ans = ans.max(r - l + 1);
    }

    (ans - 1) as i32
}

#[test]
fn test() {
    assert_eq!(longest_subarray(vec![1, 1, 0, 1]), 3);
    assert_eq!(longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
    assert_eq!(longest_subarray(vec![1, 1, 1]), 2);
}
