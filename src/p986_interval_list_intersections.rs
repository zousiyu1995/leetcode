#[test]
fn test() {
    assert_eq!(
        interval_intersection(
            vec![[0, 2], [5, 10], [13, 23], [24, 25]]
                .iter_mut()
                .map(|v| v.to_vec())
                .collect(),
            vec![[1, 5], [8, 12], [15, 24], [25, 26]]
                .iter_mut()
                .map(|v| v.to_vec())
                .collect()
        ),
        vec![[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
            .iter_mut()
            .map(|v| v.to_vec())
            .collect::<Vec<Vec<i32>>>()
    );
}

// 双指针
pub fn interval_intersection(
    first_list: Vec<Vec<i32>>,
    second_list: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut first: usize = 0; // 区间1的指针
    let mut second: usize = 0; // 区间2的指针
    let mut ans: Vec<Vec<i32>> = vec![];

    while first < first_list.len() && second < second_list.len() {
        // 右边界中最大的作为交集的右边界，左边界中最小的作为交集的左边界
        let start: i32 = first_list[first][0].max(second_list[second][0]);
        let end: i32 = first_list[first][1].min(second_list[second][1]);
        // 判断交集是否合法，因为start可能大于end
        if start <= end {
            ans.push(vec![start, end]);
        }
        // 哪个区间小（判断右边界即可），它的指针就先步进。因为区间大的还可能和别人重叠
        if first_list[first][1] < second_list[second][1] {
            first += 1;
        } else {
            second += 1;
        }
    }

    ans
}
