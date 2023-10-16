#[test]
fn test() {
    use method1::single_number;

    // 断言不能通过，因为哈希表无序
    assert_eq!(single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
    assert_eq!(single_number(vec![-1, 0]), vec![-1, 0]);
    assert_eq!(single_number(vec![0, 1]), vec![1, 0]);
}

// 哈希表，简单粗暴
mod method1 {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans: Vec<i32> = vec![];

        for num in nums {
            map.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }

        for (k, v) in map {
            if v == 1 {
                ans.push(k);
            }
        }

        ans
    }
}

// 位运算，暂时不会
