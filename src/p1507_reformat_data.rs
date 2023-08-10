#[test]
fn test() {
    let date1: String = "20th Oct 2052".to_string();
    assert_eq!(reformat_date(date1), "2052-10-20");

    let date2: String = "6th Jun 1933".to_string();
    assert_eq!(reformat_date(date2), "1933-06-06".to_string());
}

// 字符串
pub fn reformat_date(date: String) -> String {
    let date: Vec<String> = date.split_whitespace().map(|s| s.to_string()).collect();
    use std::collections::HashMap;
    let map_month: HashMap<String, i32> = [
        ("Jan", 1),
        ("Feb", 2),
        ("Mar", 3),
        ("Apr", 4),
        ("May", 5),
        ("Jun", 6),
        ("Jul", 7),
        ("Aug", 8),
        ("Sep", 9),
        ("Oct", 10),
        ("Nov", 11),
        ("Dec", 12),
    ]
    .iter()
    .map(|(s, num)| (s.to_string(), *num))
    .collect();

    let day: &i32 = &date[0]
        .chars()
        .filter(|ch| ch.is_numeric())
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    let month: &i32 = map_month.get(&date[1]).unwrap();
    let year: &i32 = &date[2].parse::<i32>().unwrap();

    format!("{:04}-{:02}-{:02}", year, month, day)
}
