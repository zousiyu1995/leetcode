#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v1: Vec<i32> = vec![2, 2, 1];
        assert_eq!(single_number(v1), 1);

        let v2: Vec<i32> = vec![4, 1, 2, 1, 2];
        assert_eq!(single_number(v2), 4);

        let v3: Vec<i32> = vec![1];
        assert_eq!(single_number(v3), 1);
    }
}

use std::collections::HashMap;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        map.entry(num)
            .and_modify(|x: &mut i32| *x += 1)
            .or_insert(1);
    }

    for (k, v) in map {
        if v == 1 {
            return k;
        }
    }

    -1
}
