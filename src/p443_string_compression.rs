#[test]
fn test1() {
    assert_eq!(compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']), 6);
    assert_eq!(compress(&mut vec!['a']), 1);
    assert_eq!(
        compress(&mut vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
        ]),
        4
    );
}

// 双指针，一个读，一个写
// ['a', 'a', 'a', 'b', 'b', 'c', 'c', 'c']
//             ^     ^    ^
//           write  left  right
// ['a', '2', x, x, ...]
pub fn compress(chars: &mut Vec<char>) -> i32 {
    let n: usize = chars.len();
    let mut left: usize = 0; // 连续字符串的左边界指针
    let mut write: usize = 0; // 写指针

    // 连续字符串的右边界指针
    for right in 0..n {
        // 右指针到达边界，或当前的字符不等于下一个字符，意味着一个连续的字符串。停止读，写入数据
        if right == n - 1 || chars[right] != chars[right + 1] {
            // 先写入字符
            chars[write] = chars[right];
            write += 1;
            // 条件：如果连续的字符长度大于1，写入长度
            //  3 -> '3'
            // 12 -> '1', '2'
            let len: usize = right - left + 1;
            if len > 1 {
                for ch in format!("{}", len).chars() {
                    chars[write] = ch;
                    write += 1;
                }
            }
            // 更新左边界指针
            left = right + 1;
        }
    }

    // write指针的大小就是新数组的长度
    write as i32
}
