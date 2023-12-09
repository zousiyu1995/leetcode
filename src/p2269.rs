#[test]
fn test() {
    assert_eq!(divisor_substrings(240, 2), 2);
    assert_eq!(divisor_substrings(430043, 2), 2);
}

pub fn divisor_substrings(num: i32, k: i32) -> i32 {
    let num_s: String = num.to_string();
    let n: usize = num_s.len();

    let mut ans: i32 = 0;

    for i in 0..=(n - k as usize) {
        let tmp: i32 = (&num_s[i..i + k as usize]).parse::<i32>().unwrap();
        if tmp != 0 && num % tmp == 0 {
            ans += 1;
        }
    }

    ans
}
