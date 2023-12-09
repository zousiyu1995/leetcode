#[test]
fn test() {
    let strs1: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(
        group_anagrams(strs1),
        vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]
        ]
    );

    let strs2: Vec<String> = vec!["".to_string()];
    assert_eq!(group_anagrams(strs2), vec![vec!["".to_string()]]);

    let strs3: Vec<String> = vec!["a".to_string()];
    assert_eq!(group_anagrams(strs3), vec![vec!["a".to_string()]]);
}

// 条件：字母全是小写
// 哈希表是无序的，所以不一定每次都能通过测试
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut map1: HashMap<[i32; 26], Vec<String>> = HashMap::new();

    for s in strs {
        // 用map2判断是否是字母异位词
        let mut map2: [i32; 26] = [0; 26];
        for ch in s.chars() {
            map2[(ch as u8 - 'a' as u8) as usize] += 1;
        }
        // 用map1收集字母异位词
        map1.entry(map2).or_insert(vec![]).push(s);
    }

    map1.values().cloned().collect()
}
