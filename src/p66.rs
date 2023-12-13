pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits: Vec<i32> = digits;
    let mut i: i32 = digits.len() as i32 - 1;
    while i >= 0 {
        // 需要进位就不断循环
        if (digits[i as usize] + 1) % 10 == 0 {
            digits[i as usize] = 0;
        }
        // 不需要进位就跳出循环
        else {
            digits[i as usize] += 1;
            return digits;
        }
        i -= 1;
    }

    if digits[0] == 0 {
        digits.insert(0, 1);
    }

    digits
}

#[test]
fn test() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(plus_one(vec![0]), vec![1]);
}
