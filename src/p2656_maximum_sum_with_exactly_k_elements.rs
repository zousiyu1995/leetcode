#[test]
fn test() {
    use method2::maximize_sum;

    assert_eq!(maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
    assert_eq!(maximize_sum(vec![5, 5, 5], 2), 11);
}

// https://leetcode.cn/problems/maximum-sum-with-exactly-k-elements/description/

mod method1 {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut max: i32 = *nums.iter().max().unwrap();
        let mut ans: i32 = 0;

        for _ in 0..k {
            ans += max;
            max += 1;
        }

        ans
    }
}

mod method2 {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let max: i32 = *nums.iter().max().unwrap();

        (2 * max + k - 1) * k / 2
    }
}
