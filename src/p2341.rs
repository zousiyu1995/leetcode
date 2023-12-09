#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1: Vec<i32> = vec![1, 3, 2, 1, 3, 2, 2];
        assert_eq!(number_of_pairs(nums1), vec![3, 1]);

        let nums2: Vec<i32> = vec![1, 1];
        assert_eq!(number_of_pairs(nums2), vec![1, 0]);

        let nums3: Vec<i32> = vec![0];
        assert_eq!(number_of_pairs(nums3), vec![0, 1]);
    }
}

use std::collections::HashMap;

pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, bool> = HashMap::new();
    let mut pairs: i32 = 0;

    for num in nums {
        // 如果找到这个key，去掉
        if map.contains_key(&num) {
            map.remove(&num);
            pairs += 1;
        }
        // 如果没找到这个key，插入
        else {
            map.insert(num, true);
        }
    }

    vec![pairs, map.len() as i32]
}
