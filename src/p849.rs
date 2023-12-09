#[test]
fn test() {
    use method1::max_dist_to_closest;

    assert_eq!(max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
    assert_eq!(max_dist_to_closest(vec![1, 0, 0, 0]), 3);
    assert_eq!(max_dist_to_closest(vec![0, 1]), 1);
    assert_eq!(max_dist_to_closest(vec![1, 0, 1]), 1);
}

// 可能会坐在三个位置
// 坐开头，[0, 0, 0, 0, 1]，距离为dist1
// 坐结尾，[1, 0, 0, 0, 0]，距离为dist2
// 坐中间，[1, 0, 0, 0, 1]，坐中间有很多情况，取最大距离dist3
// 比较哪个距离最远即可，距离实际上就是连续0的个数
// 虽然有3个循环，但是三个循环的范围分别为[0, i]、[i + 1， j]、[j, len - 1]
// 所以时间复杂度O(n)
mod method1 {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut i: usize = 0;
        let mut j: usize = seats.len() - 1;
        // 计算坐开头的距离
        let mut dist1: i32 = 0;
        while seats[i] == 0 {
            i += 1;
            dist1 += 1;
        }
        // 计算坐结尾的距离
        let mut dist2: i32 = 0;
        while seats[j] == 0 {
            j -= 1;
            dist2 += 1;
        }
        // 此时i，j指向1
        // 计算坐中间时的最大距离
        let mut dist3: i32 = 0;
        let mut tmp: i32 = 0;
        for k in (i + 1)..=j {
            if seats[k] == 0 {
                tmp += 1;
            } else {
                dist3 = dist3.max(tmp);
                tmp = 0;
            }
        }

        dist1.max(dist2).max((dist3 + 1) / 2)
    }
}
