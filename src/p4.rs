#[test]
fn test() {
    use method1::find_median_sorted_arrays;

    // assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    // assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    // assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
    // assert_eq!(find_median_sorted_arrays(vec![3], vec![-2, -1]), -1.0);
    assert_eq!(find_median_sorted_arrays(vec![1], vec![1]), 1.0);
}

// 简单粗暴法
// 先合并两个数组，合并方法用双指针
// 然后取中位数
mod method1 {
    #[allow(unused)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 用倒序双指针合并两个有序数组，借鉴第88题
        let len1: i32 = nums1.len() as i32; // nums1的长度
        let len2: i32 = nums2.len() as i32; // nums2的长度
        let mut nums1: Vec<i32> = nums1;
        nums1.extend(vec![0; len2 as usize]); // 合并后的数组

        let mut p: i32 = len1 + len2 - 1; // 指向要插入的位置
        let mut p1: i32 = len1 - 1;
        let mut p2: i32 = len2 - 1;

        // 倒序枚举nums2
        while p2 >= 0 {
            // 只要nums1的元素大，就插入nums1的元素
            while p1 >= 0 && nums1[p1 as usize] > nums2[p2 as usize] {
                nums1.swap(p as usize, p1 as usize);
                p -= 1;
                p1 -= 1;
            }
            // 否则，插入nums_l的元素
            nums1[p as usize] = nums2[p2 as usize];
            p -= 1;
            p2 -= 1;
        }

        if nums1.len() % 2 == 1 {
            nums1[nums1.len() / 2] as f64
        } else {
            (nums1[nums1.len() / 2] + nums1[nums1.len() / 2 - 1]) as f64 / 2.0
        }
    }
}

mod method2 {}
