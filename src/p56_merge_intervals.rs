#[test]
fn test() {
    let intervals1: Vec<Vec<i32>> = vec![[1, 3], [2, 6], [8, 10], [15, 18]]
        .iter()
        .map(|arr| arr.to_vec())
        .collect();
    let ans1: Vec<Vec<i32>> = vec![[1, 6], [8, 10], [15, 18]]
        .iter()
        .map(|arr| arr.to_vec())
        .collect();
    assert_eq!(merge(intervals1), ans1);

    let intervals2: Vec<Vec<i32>> = vec![[1, 4], [4, 5]]
        .iter()
        .map(|arr| arr.to_vec())
        .collect();
    let ans2: Vec<Vec<i32>> = vec![[1, 5]].iter().map(|arr| arr.to_vec()).collect();
    assert_eq!(merge(intervals2), ans2);

    assert_eq!(merge(vec![vec![1, 3]]), vec![vec![1, 3]]);
}

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals: Vec<Vec<i32>> = intervals;
    // 按左端点排序
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut ans: Vec<Vec<i32>> = vec![];

    for interval in intervals {
        // 如果ans为空，或者区间不重叠
        if ans.is_empty() || ans.last().unwrap()[1] < interval[0] {
            ans.push(interval);
        }
        // 区间重叠，合并区间
        // ans:    [1, 3]
        // interval: [2, 4]
        else {
            ans.last_mut().unwrap()[1] = ans.last().unwrap()[1].max(interval[1]);
        }
    }

    ans
}
