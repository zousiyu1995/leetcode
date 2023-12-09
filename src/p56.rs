#[test]
fn test() {
    use method2::merge;

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

// https://leetcode.cn/problems/merge-intervals/

mod method1 {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals: Vec<Vec<i32>> = intervals;
        // 按左端点排序
        intervals.sort_by(|a: &Vec<i32>, b: &Vec<i32>| a[0].cmp(&b[0]));
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
}

mod method2 {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals: Vec<Vec<i32>> = intervals;
        // 按左端点排序
        intervals.sort_by(|a: &Vec<i32>, b: &Vec<i32>| a[0].cmp(&b[0]));

        let mut ans: Vec<Vec<i32>> = vec![];
        let mut cur: usize = 0;
        while cur < intervals.len() {
            let start: i32 = intervals[cur][0]; // 因为已经拍虚了合并区间时start不变，这么些是方便阅读
            let mut end: i32 = intervals[cur][1]; // 但是end会变
            let mut next: usize = cur + 1;
            // 只要和下一个区间重叠，就合并
            while next < intervals.len() && intervals[next][0] <= end {
                end = end.max(intervals[next][1]);
                next += 1;
            }
            ans.push(vec![start, end]);
            cur = next;
        }

        ans
    }
}
