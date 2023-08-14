#[test]
fn test() {
    use method1::search_matrix;

    let matrix1: Vec<Vec<i32>> = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]]
        .map(|arr| arr.to_vec())
        .to_vec();
    assert_eq!(search_matrix(matrix1, 3), true);

    let matrix2: Vec<Vec<i32>> = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]]
        .map(|arr| arr.to_vec())
        .to_vec();
    assert_eq!(search_matrix(matrix2, 13), false);
}

// 二分查找，先变成一维数组吧
mod method1 {
    #[allow(unused)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let matrix: Vec<i32> = matrix.into_iter().flatten().collect();
        // matrix.binary_search(&target).is_ok() // 作弊，rust自带二分查找

        let (mut l, mut r): (usize, usize) = (0, matrix.len());
        // [l, r)
        while l < r {
            let m: usize = l + (r - l) / 2;
            if matrix[m] < target {
                l = m + 1;
            } else if matrix[m] > target {
                r = m;
            } else {
                return true;
            }
        }

        false
    }
}
