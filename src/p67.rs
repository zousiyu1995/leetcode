// 模拟
pub fn add_binary(a: String, b: String) -> String {
    let mut a: Vec<i32> = a
        .chars()
        .map(|c: char| c.to_digit(10).unwrap() as i32)
        .collect();
    let mut b: Vec<i32> = b
        .chars()
        .map(|c: char| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut ans: Vec<String> = vec![];
    let mut carry: i32 = 0;
    let i: i32 = a.len().max(b.len()) as i32;
    for _ in 0..i {
        let x: i32 = a.pop().unwrap_or(0);
        let y: i32 = b.pop().unwrap_or(0);
        let sum: i32 = x + y + carry;
        if sum >= 2 {
            ans.push((sum - 2).to_string());
            carry = 1;
        } else {
            ans.push(sum.to_string());
            carry = 0;
        }
    }

    if carry == 1 {
        ans.push("1".to_string());
    }

    ans.into_iter().rev().collect()
}

#[test]
fn test() {
    assert_eq!(
        add_binary("1111".to_string(), "1111".to_string()),
        "11110".to_string()
    );
    assert_eq!(
        add_binary("0".to_string(), "0".to_string()),
        "0".to_string()
    );
    assert_eq!(
        add_binary("11".to_string(), "1".to_string()),
        "100".to_string()
    );
    assert_eq!(
        add_binary("1010".to_string(), "1011".to_string()),
        "10101".to_string()
    );
}
