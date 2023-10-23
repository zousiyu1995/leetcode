#[test]
fn test() {
    assert_eq!(
        count_seniors(
            vec!["7868190130M7522", "5303914400F9211", "9273338290F4010"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        ),
        2
    );
    assert_eq!(
        count_seniors(
            vec!["1313579440F2036", "2921522980M5644"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        ),
        0
    );
}

pub fn count_seniors(details: Vec<String>) -> i32 {
    let mut count: i32 = 0;

    for s in details {
        if (&s[11..=12]).parse::<i32>().unwrap() > 60 {
            count += 1;
        }
    }

    count
}
