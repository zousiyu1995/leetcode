#[test]
fn test() {
    // assert_eq!(
    //     erase_overlap_intervals(
    //         vec![[1, 2], [2, 3], [3, 4], [1, 3]]
    //             .iter_mut()
    //             .map(|x| x.to_vec())
    //             .collect()
    //     ),
    //     1
    // );

    // assert_eq!(
    //     erase_overlap_intervals(
    //         vec![[1, 2], [1, 2], [1, 2]]
    //             .iter_mut()
    //             .map(|x| x.to_vec())
    //             .collect()
    //     ),
    //     2
    // );
    // assert_eq!(
    //     erase_overlap_intervals(
    //         vec![[1, 2], [2, 3]]
    //             .iter_mut()
    //             .map(|x| x.to_vec())
    //             .collect()
    //     ),
    //     0
    // );
    assert_eq!(
        erase_overlap_intervals(
            vec![[1, 100], [11, 22], [1, 11], [2, 12]]
                .iter_mut()
                .map(|x| x.to_vec())
                .collect()
        ),
        2
    );
}

pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals: Vec<Vec<i32>> = intervals;
    if intervals.len() == 0 {
        return 0;
    }

    // 按区间结尾排序
    intervals.sort_by(|a, b| a[1].cmp(&b[1]));

    let mut ans: i32 = 0;
    // 必须用变量记住上一个区间，因为中间可能有区间已经被移除了
    let mut last: &Vec<i32> = &intervals[0];
    for cur in intervals.iter().skip(1) {
        // 如果和前一个区域重叠，就移除
        if cur[0] < last[1] {
            ans += 1;
        } else {
            last = cur;
        }
    }

    ans
}
