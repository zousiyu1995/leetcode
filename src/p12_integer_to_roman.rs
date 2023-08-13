#[test]
fn test() {
    assert_eq!(int_to_roman(3), "III".to_string());
    assert_eq!(int_to_roman(4), "IV".to_string());
    assert_eq!(int_to_roman(9), "IX".to_string());
    assert_eq!(int_to_roman(58), "LVIII".to_string());
    assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
}

// 哈希表，贪心
pub fn int_to_roman(num: i32) -> String {
    let mut num: i32 = num;
    // rust 的 hashmap 是无序的，btreemap 是正序的，用数组当 hashmap 吧
    let map: [(i32, &str); 13] = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut ans: String = String::new();

    // 尽可能优先使用 map 中的较大数值
    // 比如，1994 这个数，每次选取尽可能大的数，依次选用 1000、900、90、4，得到答案 MCMXCIV
    for (k, v) in map {
        while num >= k {
            ans.push_str(v);
            num -= k;
        }
    }

    ans
}
