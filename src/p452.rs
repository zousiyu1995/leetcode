#[test]
fn test() {
    assert_eq!(
        find_min_arrow_shots(
            vec![[10, 16], [2, 8], [1, 6], [7, 12]]
                .iter_mut()
                .map(|x| x.to_vec())
                .collect()
        ),
        2
    );

    assert_eq!(
        find_min_arrow_shots(
            vec![[1, 2], [3, 4], [5, 6], [7, 8]]
                .iter_mut()
                .map(|x| x.to_vec())
                .collect()
        ),
        4
    );

    assert_eq!(
        find_min_arrow_shots(
            vec![[1, 2], [2, 3], [3, 4], [4, 5]]
                .iter_mut()
                .map(|x| x.to_vec())
                .collect()
        ),
        2
    );
}

// 贪心
// 问题可以转化成求这些区间的交集的数量
pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    // 排序
    let mut points: Vec<Vec<i32>> = points;
    points.sort_by(|a, b| a[1].cmp(&b[1]));

    // 初始化待发射区域
    let mut range: Vec<i32> = points[0].clone();
    let mut arrows: i32 = 1;

    for point in points.iter().skip(1) {
        // 当前区域与待发射区域存在交集，更新交集区域
        if point[0] <= range[1] {
            range[0] = range[0].max(point[0]);
            range[1] = range[1].min(point[1]);
        }
        // 没有交集，更新箭头数量，并将发射区域重置为当前区间
        else {
            arrows += 1;
            range = point.clone();
        }
    }

    arrows
}
