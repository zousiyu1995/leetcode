#[test]
fn test() {
    use method1::h_index;

    assert_eq!(h_index(vec![3, 0, 6, 1, 5]), 3);
    assert_eq!(h_index(vec![1, 3, 1]), 1);
}

// 排序，然后从大到小查找
mod method1 {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations: Vec<i32> = citations;
        // [5, 4, 3, 2, 1]
        citations.sort_by(|a, b| b.cmp(a));

        // 引用次数>h_index，h_index+1
        let mut h_index: i32 = 0;
        for citation in citations {
            if citation > h_index {
                h_index += 1;
            }
        }

        h_index
    }
}

// 二分查找，暂时没搞懂
#[allow(unused)]
mod method2 {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        todo!()
    }
}
