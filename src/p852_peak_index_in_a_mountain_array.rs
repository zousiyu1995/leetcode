#[test]
fn test() {
    assert_eq!(peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    assert_eq!(peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
}

// 二分查找
// 条件：峰值不在0和len-1的位置；len>=3
// 如果mid<mid+1，缩左边界
// 如果mid>mid+1，缩右边界
pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut left: usize = 1;
    let mut right: usize = arr.len() - 1;
    // [left, right)
    while left < right {
        let mid: usize = left + (right - left) / 2;
        if arr[mid] < arr[mid + 1] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left as i32
}
