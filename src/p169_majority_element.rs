#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v1: Vec<i32> = vec![3, 2, 3];
        assert_eq!(majority_element(v1), 3);

        let v2: Vec<i32> = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(majority_element(v2), 2);
    }
}

use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in nums.iter() {
        map.entry(*num)
            .and_modify(|x: &mut i32| *x += 1)
            .or_insert(1);
    }

    for (k, v) in map {
        if v > (nums.len() / 2) as i32 {
            return k;
        }
    }

    0
}
