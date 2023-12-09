#[test]
fn test() {
    assert_eq!(my_atoi("42".to_string()), 42);
    assert_eq!(my_atoi("   -42".to_string()), -42);
    assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(my_atoi("words and 987".to_string()), 0);
    assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(my_atoi("2147483648".to_string()), 2147483647);
    assert_eq!(my_atoi("9223372036854775808".to_string()), 2147483647);
}

pub fn my_atoi(s: String) -> i32 {
    let mut num: i64 = 0;
    let mut sym: i64 = 1;

    for (i, ch) in s.trim().chars().enumerate() {
        // 先处理符号
        if ch == '-' && i == 0 {
            sym *= -1;
            continue;
        } else if ch == '+' && i == 0 {
            sym *= 1;
            continue;
        }
        // 处理非数字
        else if !ch.is_digit(10) {
            break;
        }
        // 处理数字
        else {
            num = num * 10 + (ch.to_digit(10).unwrap() as i64);
            // 检查是否超出i32的范围
            if num * sym > i32::MAX as i64 {
                return i32::MAX;
            } else if num * sym < i32::MIN as i64 {
                return i32::MIN;
            }
        }
    }

    (num * sym) as i32
}
