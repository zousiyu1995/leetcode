#[test]
fn test() {
    use method2::longest_consecutive;

    assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
}

// 哈希表
// 遍历数组，遇到num，去哈希表中找num+x，直到其不存在，x+1即为最大长度
// 时间复杂度O(N^2)，会超时
mod method1 {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();
        for &num in nums.iter() {
            set.insert(num);
        }

        let mut ans: i32 = 0;
        for &num in nums.iter() {
            let mut target: i32 = num + 1; // num+x
            while set.contains(&target) {
                target += 1;
            }
            ans = ans.max(target - num);
        }

        ans
    }
}

// 只需要稍微改一下method1，时间复杂度就能达到O(N)
mod method2 {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();
        for &num in nums.iter() {
            set.insert(num);
        }

        let mut ans: i32 = 0;
        for &num in nums.iter() {
            let mut target: i32 = num + 1;
            // 只有当set中不存在num-1时，才去set中找num+1, ..., num+x
            // 这个if保证了每次从连续序列的最小值开始找num+x
            if !set.contains(&(num - 1)) {
                while set.contains(&target) {
                    target += 1;
                }
            }
            ans = ans.max(target - num);
        }

        ans
    }
}

// method2的效率还是不够高
mod method3 {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        unimplemented!()
    }
}
