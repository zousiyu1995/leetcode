#[test]
fn test() {
    use method1::first_missing_positive;

    assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}

// 将nums放入哈希集中，然后从1开始枚举正整数，第一个不在哈希集中的正整数就是答案
// 空间复杂度是O(N)，其实不满足要求
mod method1 {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();
        for num in nums {
            set.insert(num);
        }

        let mut missing: i32 = 0;
        for num in 1..=i32::MAX {
            if !set.contains(&num) {
                missing = num;
                break;
            }
        }

        missing
    }
}

// 还有其他方法，以后再说
