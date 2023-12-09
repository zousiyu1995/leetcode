#[test]
fn test() {
    use method2::convert;

    assert_eq!(
        convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
        convert("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI".to_string()
    );
    assert_eq!(convert("A".to_string(), 1), "A".to_string());
}

// 模拟法
mod method1 {
    #[allow(unused)]
    pub fn convert(s: String, num_rows: i32) -> String {
        let n: i32 = s.len() as i32; // 字符串长度
        let row: i32 = num_rows;
        // 特殊情况
        if row == 1 || row >= n as i32 {
            return s;
        }

        // 变换周期是2*row-2
        let period: i32 = 2 * row - 2;
        let col: i32 = (n + period - 1) / period * (row - 1);
        let mut mat = vec![vec!["".to_string(); col as usize]; row as usize];

        // 将字符填入matrix
        let (mut i, mut j): (usize, usize) = (0, 0);
        for (idx, ch) in s.chars().enumerate() {
            mat[i][j] = ch.to_string();
            if idx as i32 % period < row - 1 {
                i += 1;
            } else {
                i -= 1;
                j += 1;
            }
        }

        // 依次从matrix中取出字符，拼接
        let mut ans: String = String::new();
        for v in mat {
            for s in v {
                ans.push_str(&s);
            }
        }

        ans
    }
}

// 观察规律，设输出的行数是rows，字符依次会被放入第012101210...行，循环往复直至字符被放完
mod method2 {
    #[allow(unused)]
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows: usize = num_rows as usize;
        let mut rows: Vec<String> = vec![String::new(); num_rows];
        // 创建循环迭代器，比如，0123210123210......
        let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
        // 按迭代器给出的下标访问对应行，推入字符
        iter.zip(s.chars()).for_each(|(idx, c)| rows[idx].push(c));
        // collect连接每行
        rows.into_iter().collect()
    }
}
