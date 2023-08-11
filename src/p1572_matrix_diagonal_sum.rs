#[test]
fn test() {
    use method2::diagonal_sum;

    let mat1: Vec<Vec<i32>> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .map(|arr| arr.to_vec())
        .to_vec();
    assert_eq!(diagonal_sum(mat1), 25);

    let mat2: Vec<Vec<i32>> = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        .map(|arr| arr.to_vec())
        .to_vec();
    assert_eq!(diagonal_sum(mat2), 8);
}

#[allow(dead_code)]
mod method1 {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        // matrix size is nxn
        let mut sum: i32 = 0;
        let n: usize = mat[0].len();

        // 主对角线：i = j
        // 副对角线：i + j = n - 1
        // 遍历每一个元素
        for i in 0..n {
            for j in 0..n {
                if i == j || i + j == n - 1 {
                    sum += mat[i][j];
                }
            }
        }

        sum
    }
}

#[allow(dead_code)]
mod method2 {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        // matrix size is nxn
        let mut sum: i32 = 0;
        let n: usize = mat[0].len();

        // 主对角线：i = j
        // 副对角线：i + j = n - 1
        // 只遍历行
        for i in 0..n {
            sum += mat[i][i] + mat[i][n - 1 - i];
        }

        if n % 2 == 1 {
            sum - mat[n / 2][n / 2]
        } else {
            sum
        }
    }
}
