#[test]
fn test() {
    let words1: Vec<String> = vec!["are", "amy", "u"]
        .iter_mut()
        .map(|x| x.to_string())
        .collect();
    assert_eq!(vowel_strings(words1, 0, 2), 2);

    let words2: Vec<String> = vec!["hey", "aeo", "mu", "ooo", "artro"]
        .iter_mut()
        .map(|x| x.to_string())
        .collect();
    assert_eq!(vowel_strings(words2, 1, 4), 3);
}

pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    let mut ans: i32 = 0;
    let map: [&str; 5] = ["a", "e", "i", "o", "u"];

    for i in left..=right {
        let s: &String = &words[i as usize];
        let n: usize = s.len();
        if map.contains(&&s[0..1]) && map.contains(&&s[(n - 1)..n]) {
            ans += 1;
        }
    }

    ans
}
