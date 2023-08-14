#[test]
fn test() {
    let mut matrix1: Vec<Vec<i32>> = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
        .map(|arr| arr.to_vec())
        .to_vec();
    let ans1: Vec<Vec<i32>> = [[1, 0, 1], [0, 0, 0], [1, 0, 1]]
        .map(|arr| arr.to_vec())
        .to_vec();
    set_zeroes(&mut matrix1);
    assert_eq!(matrix1, ans1);

    let mut matrix2: Vec<Vec<i32>> = [[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]]
        .map(|arr| arr.to_vec())
        .to_vec();
    let ans2: Vec<Vec<i32>> = [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]
        .map(|arr| arr.to_vec())
        .to_vec();
    set_zeroes(&mut matrix2);
    assert_eq!(matrix2, ans2);
}

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let m: usize = matrix.len(); // 行
    let n: usize = matrix[0].len(); // 列
    let mut row: Vec<bool> = vec![false; m]; // 标记行是否包含0
    let mut col: Vec<bool> = vec![false; n]; // 标记列是否包含0

    // 标记行、列是否出现0
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                row[i] = true;
                col[j] = true;
            }
        }
    }

    // 将出现0的行、列置为0
    for i in 0..m {
        for j in 0..n {
            if row[i] == true || col[j] == true {
                matrix[i][j] = 0;
            }
        }
    }
}
