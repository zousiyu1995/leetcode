#[test]
fn test() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
}

// 对撞双指针，关键是有序
// 每次将max和min相加
// 如果>target，说明max和数组内其他任何元素相加>target，去掉max
// 如果<target，说明min和数组内其他任何元素相加<target，去掉min
// 如果=target，显然就是答案
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l: usize = 0;
    let mut r: usize = numbers.len() - 1;

    while l < r {
        let sum: i32 = numbers[l] + numbers[r];
        if sum > target {
            r -= 1;
        } else if sum < target {
            l += 1;
        } else {
            // 数组下标从 1 开始，注意加 1
            return vec![(l + 1) as i32, (r + 1) as i32];
        }
    }

    // 没找到
    vec![0, 0]
}
