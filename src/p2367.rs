#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1: Vec<i32> = vec![0, 1, 4, 6, 7, 10];
        assert_eq!(arithmetic_triplets(nums1, 3), 2);

        let nums2: Vec<i32> = vec![4, 5, 6, 7, 8, 9];
        assert_eq!(arithmetic_triplets(nums2, 2), 2);
    }
}

use std::collections::HashMap;

pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    let mut map: HashMap<i32, bool> = HashMap::new();
    let mut ans: i32 = 0;

    for num in &nums {
        map.entry(*num).or_insert(true);
    }

    for i in &nums {
        if map.contains_key(&(i + diff)) && map.contains_key(&(i + 2 * diff)) {
            ans += 1;
        }
    }

    ans
}
