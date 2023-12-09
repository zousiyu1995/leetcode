#[test]
fn test() {
    use method2::maximum_sum;

    assert_eq!(maximum_sum(vec![18, 43, 36, 13, 7]), 54);
    assert_eq!(maximum_sum(vec![10, 12, 19, 14]), -1);
}

// 哈希表+最大堆
mod method1 {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        use std::collections::{BinaryHeap, HashMap};

        // 键：数位和；值：数位和相等的数组成的堆
        let mut map: HashMap<i32, BinaryHeap<i32>> = HashMap::new();
        for num in nums {
            // 求数位和
            let mut tmp: i32 = num;
            let mut sum: i32 = 0;
            while tmp != 0 {
                sum += tmp % 10;
                tmp /= 10;
            }
            map.entry(sum)
                .and_modify(|v: &mut BinaryHeap<i32>| v.push(num))
                .or_insert(BinaryHeap::from([num]));
        }

        let mut ans: i32 = -1;
        // 如果堆的长度大于2，从堆顶取两个元素出来，更新ans
        for (_, v) in map.iter_mut() {
            if v.len() >= 2 {
                let mut sum: i32 = 0;
                sum += v.pop().unwrap();
                sum += v.pop().unwrap();
                ans = ans.max(sum);
            }
        }

        ans
    }
}

// 在method1的基础上优化，哈希表内没必要保存数位和相等的全部元素，维护一个最大的元素就行了
mod method2 {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans: i32 = -1;
        for num in nums {
            // 求数位和
            let mut tmp: i32 = num;
            let mut sum: i32 = 0;
            while tmp != 0 {
                sum += tmp % 10;
                tmp /= 10;
            }
            // 如果有数位和相等的元素，更新ans
            if map.contains_key(&sum) {
                ans = ans.max(map.get(&sum).unwrap() + num);
            }
            // 仅维护数位和为sum的最大元素
            map.entry(sum)
                .and_modify(|v: &mut i32| *v = *v.max(&mut num.clone()))
                .or_insert(num);
        }

        ans
    }
}
