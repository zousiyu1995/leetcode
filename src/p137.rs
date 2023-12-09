#[test]
fn test() {
    use method1::single_number;

    assert_eq!(single_number(vec![2, 2, 3, 2]), 3);
    assert_eq!(single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
}

// 哈希表
mod method1 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            map.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }

        for (k, v) in map {
            if v == 1 {
                return k;
            }
        }

        -1
    }
}

// 位运算，暂时不会
