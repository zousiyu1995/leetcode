#[test]
fn test() {
    use method2::missing_number;

    assert_eq!(missing_number(vec![3, 0, 1]), 2);
    assert_eq!(missing_number(vec![0, 1]), 2);
    assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
}

// 数学题
// [0, n]共n+1个数求和，减去nums的和，余下的就是丢失的数字
mod method1 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n: i32 = nums.len() as i32;
        let sum: i32 = n * (1 + n) / 2;

        sum - nums.iter().sum::<i32>()
    }
}

// 哈希集
// 遍历nums，将每个元素加入哈希集
// 遍历[0..n]，检查每个元素是否在哈希集中
mod method2 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n: i32 = nums.len() as i32;

        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();
        for num in nums {
            set.insert(num);
        }

        let mut missing: i32 = -1;
        for num in 0..=n {
            if !set.contains(&num) {
                missing = num;
            }
        }

        missing
    }
}
