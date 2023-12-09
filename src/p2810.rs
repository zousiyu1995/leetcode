#[test]
fn test() {
    assert_eq!(final_string("string".to_string()), "rtsng".to_string());
    assert_eq!(final_string("poiinter".to_string()), "ponter".to_string());
}

pub fn final_string(s: String) -> String {
    // 双端队列
    use std::collections::VecDeque;

    let mut ans: VecDeque<char> = VecDeque::new();
    let mut tail: bool = true;
    for ch in s.chars() {
        // 改变顺序
        if ch == 'i' {
            tail = !tail;
        }
        // 往后端加
        else if tail {
            ans.push_back(ch);
        }
        // 往前端加
        else {
            ans.push_front(ch);
        }
    }

    if tail {
        ans.iter().collect()
    } else {
        ans.iter().rev().collect()
    }
}
