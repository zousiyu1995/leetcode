#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut arr1: [i32; 4] = [3, 2, 2, 3];
        let val1: i32 = 3;
        assert_eq!(remove_element(&mut arr1, val1), 2);

        let mut arr2: [i32; 8] = [0, 1, 2, 2, 3, 0, 4, 2];
        let val2: i32 = 2;
        assert_eq!(remove_element(&mut arr2, val2), 5);
        println!("new arr: {:?}", arr2);
    }
}

pub fn remove_element(arr: &mut [i32], val: i32) -> i32 {
    let mut slow: usize = 0; // 慢指针

    // 移动快指针
    // 如果快指针指向的元素不是要删除的元素，将其指向的元素填入慢指针指向的元素，同时慢指针+1
    // 如果快指针指向的元素是要删除的元素，快指针+1，慢指针不动
    for fast in 0..arr.len() {
        if arr[fast] != val {
            arr[slow] = arr[fast];
            slow += 1;
        }
    }

    slow as i32
}
