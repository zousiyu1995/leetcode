#[test]
fn test() {
    use method1::rotate;

    let mut matrix1: Vec<Vec<i32>> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .map(|arr| arr.to_vec())
        .to_vec();
    rotate(&mut matrix1);
    let ans1: Vec<Vec<i32>> = [[7, 4, 1], [8, 5, 2], [9, 6, 3]]
        .map(|arr| arr.to_vec())
        .to_vec();
    assert_eq!(matrix1, ans1);

    let mut matrix2: Vec<Vec<i32>> = [
        [5, 1, 9, 11],
        [2, 4, 8, 10],
        [13, 3, 6, 7],
        [15, 14, 12, 16],
    ]
    .map(|arr| arr.to_vec())
    .to_vec();
    rotate(&mut matrix2);
    let ans2: Vec<Vec<i32>> = [
        [15, 13, 2, 5],
        [14, 3, 4, 1],
        [12, 6, 8, 9],
        [16, 7, 10, 11],
    ]
    .map(|arr| arr.to_vec())
    .to_vec();
    assert_eq!(matrix2, ans2);
}

// 数学问题
// 先上下翻转，然后沿主对角线翻转
mod method1 {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n: usize = matrix.len();
        // 先上下翻转，第 i 行和第 n - i - 1 行交换
        // matrix[i][..] <-> matrix[n - i - 1][..]
        for i in 0..(n / 2) {
            matrix.swap(i, n - i - 1);
        }

        // 然后沿主对角线翻转，第i 行 j 列与第 j 行 i 列交换
        // matrix[i][j] <-> matrix[j][i]
        for i in 0..n {
            // 遍历上三角矩阵
            for j in i + 1..n {
                let tmp: i32 = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }
}
