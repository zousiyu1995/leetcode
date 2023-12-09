#[test]
fn test() {
    assert_eq!(convert_to_title(1), "A");
    assert_eq!(convert_to_title(28), "AB");
    assert_eq!(convert_to_title(701), "ZY");
    assert_eq!(convert_to_title(2147483647), "FXSHRXW");
}

// 数学，进制转换
pub fn convert_to_title(column_number: i32) -> String {
    let mut column_number: i32 = column_number;
    let alphabet: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let mut ans: String = String::new();

    while column_number > 0 {
        column_number -= 1;
        let rem: i32 = column_number % 26;
        column_number /= 26;
        ans.insert(0, alphabet[rem as usize]);
    }

    ans
}
