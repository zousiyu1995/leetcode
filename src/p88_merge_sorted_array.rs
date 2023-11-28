#[test]
fn test1() {
    use method2::merge;

    let mut nums1_1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let mut nums2_1: Vec<i32> = vec![2, 5, 6];
    merge(&mut nums1_1, 3, &mut nums2_1, 3);
    assert_eq!(nums1_1, vec![1, 2, 2, 3, 5, 6]);

    let mut nums1_2: Vec<i32> = vec![0];
    let mut nums2_2: Vec<i32> = vec![1];
    merge(&mut nums1_2, 0, &mut nums2_2, 1);
    assert_eq!(nums1_2, vec![1]);

    let mut nums1_3: Vec<i32> = vec![4, 5, 6, 0, 0, 0];
    let mut nums2_3: Vec<i32> = vec![1, 2, 3];
    merge(&mut nums1_3, 3, &mut nums2_3, 3);
    assert_eq!(nums1_3, vec![1, 2, 3, 4, 5, 6]);
}

/// 先合并，再排序，暴力直接
mod method1 {
    #[allow(unused)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n): (usize, usize) = (m as usize, n as usize);
        for i in 0..n {
            nums1[m + i] = nums2[i];
        }
        nums1.sort();
    }
}

/// 倒序双指针
/// 依次从两个数组的后面选最大的填入 nums1 的末尾
mod method2 {
    #[allow(unused)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // usize - 1 时容易越界，指针全用 i32 类型
        let mut p: i32 = m + n - 1; // 指向要填入元素的位置
        let mut p1: i32 = m - 1; // 指向 nums1
        let mut p2: i32 = n - 1; // 指向 nums2

        // 倒序枚举nums2，逻辑是无论如何nums2都要被复制完，外层循环次数就是nums2的大小
        while p2 >= 0 {
            // 倒序枚举nums1，并且枚举的元素必须大于nums2中枚举的元素
            while p1 >= 0 && nums1[p1 as usize] > nums2[p2 as usize] {
                nums1.swap(p as usize, p1 as usize);
                p -= 1;
                p1 -= 1;
            }
            // 否则
            nums1[p as usize] = nums2[p2 as usize];
            p -= 1;
            p2 -= 1;
        }
    }
}

// 和method2一个思路，换一种写法
mod method3 {
    #[allow(unused)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p: i32 = m + n - 1; // 指向要填入元素的位置
        let mut p1: i32 = m - 1; // 指向 nums1
        let mut p2: i32 = n - 1; // 指向 nums2

        while p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[p as usize] = nums1[p1 as usize];
                p -= 1;
                p1 -= 1;
            } else {
                nums1[p as usize] = nums2[p2 as usize];
                p -= 1;
                p2 -= 1;
            }
        }

        // 如果nums1没有被复制完，不用管；如果nums2没有被复制完，继续复制
        while p2 >= 0 {
            nums1[p as usize] = nums2[p2 as usize];
            p -= 1;
            p2 -= 1;
        }
    }
}
