#[test]
fn test() {
    assert_eq!(
        partition_labels("ababcbacadefegdehijhklij".to_string()),
        vec![9, 7, 8]
    );
    assert_eq!(partition_labels("eccbbbbdec".to_string()), vec![10]);
}

// 要求：同一个字母只能出现在同一个片段
pub fn partition_labels(s: String) -> Vec<i32> {
    use std::collections::HashMap;

    // 记录每个字符最后出现的位置
    let mut map: HashMap<char, usize> = HashMap::new();
    for (idx, c) in s.char_indices() {
        map.insert(c, idx);
    }

    let mut ans: Vec<usize> = vec![];
    let mut farthest_idx: usize = 0;
    let mut start: usize = 0;
    // 在所有已扫描的字符中，维护这些字符最后出现的位置中最远的那一个位置
    for (idx, c) in s.char_indices() {
        farthest_idx = farthest_idx.max(*map.get(&c).unwrap());
        if idx == farthest_idx {
            ans.push(idx - start + 1);
            start = idx + 1;
        }
    }

    ans.iter_mut().map(|x: &mut usize| *x as i32).collect()
}
