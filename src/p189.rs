#[test]
fn test() {
    use method2::rotate;

    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut v1, 3);
    assert_eq!(v1, vec![5, 6, 7, 1, 2, 3, 4]);
}

// https://leetcode.cn/problems/rotate-array/

// 用额外的数组
// 下标为i的元素会出现在下标为(i+k) % n的位置
mod method1 {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k: usize = k as usize;
        let n: usize = nums.len();
        let mut ans: Vec<i32> = vec![0; n];

        for i in 0..n {
            ans[(i + k) % n] = nums[i];
        }

        for i in 0..n {
            nums[i] = ans[i];
        }
    }
}

// 数组翻转
// 先翻转所有元素，然后反转[0, k % n-1]区间的元素，最后翻转[k % n, n - 1]区间的元素
mod method2 {
    pub fn reverse(nums: &mut Vec<i32>, mut start: i32, mut end: i32) {
        while start < end {
            nums.swap(start as usize, end as usize);
            start += 1;
            end -= 1;
        }
    }

    // 实际上rust的Vector自带reverse方法，自己实现reverse是为了了解这个方法的原理
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n: i32 = nums.len() as i32;
        let k: i32 = k % n;

        reverse(nums, 0, n - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, n - 1);
    }
}
