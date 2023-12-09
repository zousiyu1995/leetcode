#[test]
fn test() {
    use method2::find_disappeared_numbers;

    assert_eq!(
        find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
    assert_eq!(find_disappeared_numbers(vec![1, 1]), vec![2]);
}

// 哈希表
// 空间复杂度O(N)
mod method1 {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut map: Vec<i32> = vec![0; nums.len()];
        for num in nums {
            map[(num - 1) as usize] += 1;
        }

        let mut ans: Vec<i32> = vec![];
        for (idx, &v) in map.iter().enumerate() {
            if v == 0 {
                ans.push((idx + 1) as i32);
            }
        }

        ans
    }
}

// 原地哈希
// 空间复杂度O(1)
mod method2 {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums: Vec<i32> = nums;
        let mut ans: Vec<i32> = vec![];

        // 以当前位置的数为索引，将对应位置的元素取反
        for i in 0..nums.len() {
            let idx: usize = nums[i].abs() as usize - 1;
            if nums[idx] > 0 {
                nums[idx] = -nums[idx];
            }
        }

        // 留下来的正数所对应的索引就是不存在的元素
        for i in 0..nums.len() {
            if nums[i] > 0 {
                ans.push(i as i32 + 1);
            }
        }

        ans
    }
}
