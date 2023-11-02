#[test]
fn test() {
    assert_eq!(count_points("B0B6G0R6R0R6G9".to_string()), 1);
    assert_eq!(count_points("B0R0G0R9R0B0G0".to_string()), 1);
    assert_eq!(count_points("G4".to_string()), 0);
}

pub fn count_points(rings: String) -> i32 {
    let n: usize = rings.len();

    use std::collections::HashMap;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    // 遍历字符串，统计每个杆上有多少颜色
    // key: num, value: color
    for i in (0..n).step_by(2) {
        let color: &str = &rings[i..i + 1];
        let num: &str = &rings[i + 1..i + 2];

        map.entry(num.to_owned())
            .and_modify(|v| {
                if !v.contains(&color.to_owned()) {
                    v.push(color.to_owned())
                }
            })
            .or_insert(vec![color.to_owned()]);
    }

    let mut ans: i32 = 0;
    for (_, v) in map {
        if v.len() == 3 {
            ans += 1;
        }
    }

    ans
}
