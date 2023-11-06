#[test]
fn test() {
    use method1::majority_element;

    assert_eq!(majority_element(vec![3, 2, 3]), vec![3]);
    assert_eq!(majority_element(vec![1]), vec![1]);
    assert_eq!(majority_element(vec![1, 2]), vec![1, 2]);
}

// 最朴素的方法是哈希表
mod method1 {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();

        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            map.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut ans: Vec<i32> = vec![];
        for (k, v) in map {
            if v > (n as i32 / 3) {
                ans.push(k);
            }
        }

        ans
    }
}

// 摩尔投票，暂时不会
mod method2 {}
