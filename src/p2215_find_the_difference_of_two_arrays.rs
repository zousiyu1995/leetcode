#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1_1: Vec<i32> = vec![1, 2, 3];
        let nums2_1: Vec<i32> = vec![2, 4, 6];
        assert_eq!(
            find_difference(nums1_1, nums2_1),
            vec![vec![1, 3], vec![4, 6]]
        );

        let nums1_2: Vec<i32> = vec![1, 2, 3, 3];
        let nums2_2: Vec<i32> = vec![1, 1, 2, 2];
        assert_eq!(find_difference(nums1_2, nums2_2), vec![vec![3], vec![]]);
    }
}

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    let set1: HashSet<i32> = nums1.iter().map(|&x| x).collect();
    let set2: HashSet<i32> = nums2.iter().map(|&x| x).collect();
    let mut ans: Vec<Vec<i32>> = vec![vec![], vec![]];

    for num in nums1 {
        if !set2.contains(&num) && !ans[0].contains(&num) {
            ans[0].push(num);
        }
    }
    for num in nums2 {
        if !set1.contains(&num) && !ans[1].contains(&num) {
            ans[1].push(num);
        }
    }

    ans
}
