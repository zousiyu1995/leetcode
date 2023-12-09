#[test]
fn test() {
    assert_eq!(
        successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
        vec![4, 0, 3]
    );
    assert_eq!(
        successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
        vec![2, 0, 2]
    );
}

// 先排序，然后相当于找能够满足条件的最小值，用二分查找即可
pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut potions: Vec<i32> = potions;
    potions.sort();

    // 找满足spells[i] * potions[j] >= success的第一个potions[j]
    // 尽可能往右找
    let mut ans: Vec<i32> = vec![];
    for spell in spells {
        let mut left: usize = 0;
        let mut right: usize = potions.len();
        // 二分查找，[left, right)
        while left < right {
            let mid: usize = left + (right - left) / 2;
            if (spell as i64 * potions[mid] as i64) < success {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.push((potions.len() - left) as i32);
    }

    ans
}
