#[test]
fn test() {
    assert_eq!(check_zero_ones("1101".to_string()), true);
    assert_eq!(check_zero_ones("111000".to_string()), false);
    assert_eq!(check_zero_ones("110100010".to_string()), false);
    assert_eq!(check_zero_ones("01111110".to_string()), true);
}

pub fn check_zero_ones(s: String) -> bool {
    let mut cnt0: i32 = 0;
    let mut cnt1: i32 = 0;
    let mut max_cnt0: i32 = 0;
    let mut max_cnt1: i32 = 0;

    for c in s.chars() {
        match c {
            '0' => {
                cnt1 = 0;
                cnt0 += 1;
                max_cnt0 = max_cnt0.max(cnt0);
            }
            '1' => {
                cnt0 = 0;
                cnt1 += 1;
                max_cnt1 = max_cnt1.max(cnt1);
            }
            _ => unreachable!(),
        }
    }

    max_cnt1 > max_cnt0
}
