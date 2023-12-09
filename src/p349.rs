#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
        assert_eq!(intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
    }
}

use std::collections::HashSet;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = nums1.into_iter().collect();
    let set2: HashSet<i32> = nums2.into_iter().collect();

    set1.intersection(&set2).map(|&x| x).collect()
}
