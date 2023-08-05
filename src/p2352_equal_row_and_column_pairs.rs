#[test]
fn test() {
    let grid1: Vec<Vec<i32>> = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
    assert_eq!(equal_pairs(grid1), 1);

    let grid2: Vec<Vec<i32>> = vec![
        vec![3, 1, 2, 2],
        vec![1, 4, 4, 5],
        vec![2, 4, 2, 2],
        vec![2, 4, 2, 2],
    ];
    assert_eq!(equal_pairs(grid2), 3);
}

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    // 将行加入hashmap
    let mut map: HashMap<&Vec<i32>, i32> = HashMap::new();
    grid.iter()
        .for_each(|row| *map.entry(row).or_insert(0) += 1);

    let mut count: i32 = 0;
    // 遍历列
    for j in 0..grid.len() {
        let mut col: Vec<i32> = vec![];
        for i in 0..grid.len() {
            col.push(grid[i][j]);
        }
        if map.contains_key(&col) {
            count += map.get(&col).unwrap();
        }
    }

    count
}
