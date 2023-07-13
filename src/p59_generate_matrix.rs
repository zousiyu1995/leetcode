#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );

        assert_eq!(
            generate_matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );

        assert_eq!(generate_matrix(1), vec![vec![1]]);
    }
}

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    // r和b不断减小，可能会越界，用开区间
    let mut l: usize = 0;
    let mut r: usize = n as usize;
    let mut t: usize = 0;
    let mut b: usize = n as usize;
    let mut mat: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];

    let mut num: i32 = 1;
    while num <= n.pow(2) {
        // left -> right
        for i in l..r {
            mat[t][i] = num;
            num += 1;
        }
        t += 1;

        // top -> bottom
        for i in t..b {
            mat[i][r - 1] = num;
            num += 1;
        }
        r -= 1;

        // right -> left
        for i in (l..r).rev() {
            mat[b - 1][i] = num;
            num += 1;
        }
        b -= 1;

        // bottom -> top
        for i in (t..b).rev() {
            mat[i][l] = num;
            num += 1;
        }
        l += 1;
    }

    mat
}
