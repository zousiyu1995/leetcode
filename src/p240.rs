#[test]
fn test() {
    use method1::search_matrix;

    let mat1: Vec<Vec<i32>> = vec![
        [1, 4, 7, 11, 15],
        [2, 5, 8, 12, 19],
        [3, 6, 9, 16, 22],
        [10, 13, 14, 17, 24],
        [18, 21, 23, 26, 30],
    ]
    .iter_mut()
    .map(|v| v.to_vec())
    .collect();
    assert_eq!(search_matrix(mat1, 5), true);

    let mat2: Vec<Vec<i32>> = vec![
        [1, 4, 7, 11, 15],
        [2, 5, 8, 12, 19],
        [3, 6, 9, 16, 22],
        [10, 13, 14, 17, 24],
        [18, 21, 23, 26, 30],
    ]
    .iter_mut()
    .map(|v| v.to_vec())
    .collect();
    assert_eq!(search_matrix(mat2, 20), false);
}

// 每行的所有元素从左到右升序排列
// 每列的所有元素从上到下升序排列
// 从右上角开始找
mod method1 {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m: usize = matrix.len();
        let n: usize = matrix[0].len();
        let mut i: usize = 0;
        let mut j: usize = n - 1;

        while i < m && j < n {
            // 当前值小了，向下找
            if matrix[i][j] < target {
                i += 1;
            }
            // 当前值大了，向左找
            else if matrix[i][j] > target {
                j -= 1;
            } else {
                return true;
            }
        }

        false
    }
}

// 二分搜索
mod method2 {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        todo!()
    }
}
