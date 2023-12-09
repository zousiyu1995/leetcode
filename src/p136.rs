#[test]
fn test() {
    use method2::single_number;

    let v1: Vec<i32> = vec![2, 2, 1];
    assert_eq!(single_number(v1), 1);

    let v2: Vec<i32> = vec![4, 1, 2, 1, 2];
    assert_eq!(single_number(v2), 4);

    let v3: Vec<i32> = vec![1];
    assert_eq!(single_number(v3), 1);
}

// 哈希表
mod method1 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

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
}

// 异或运算性质
// 1. 任何数和0做异或运算，结果仍然是原来的数
// 2. 任何数和其自身做异或运算，结果是0
// 3. 异或运算满足交换律和结合律
// 对数组中全部元素做异或运算，相当于对出现两次的数分别做异或（得0），然后将0和出现一次的数做异或（得这个数本身）
// 因此，数组中的全部元素的异或运算结果即为数组中只出现一次的数字
mod method2 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans: i32 = 0;

        for i in nums {
            ans ^= i;
        }

        ans
    }
}
