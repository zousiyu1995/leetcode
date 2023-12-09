#[test]
fn test() {
    use method2::insert;

    assert_eq!(
        insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        vec![vec![1, 5], vec![6, 9]]
    );
    assert_eq!(
        insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ],
            vec![4, 8]
        ),
        vec![vec![1, 2], vec![3, 10], vec![12, 16]]
    );
    assert_eq!(insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
    assert_eq!(insert(vec![vec![1, 5]], vec![2, 5]), vec![vec![1, 5]]);
    assert_eq!(insert(vec![vec![1, 5]], vec![2, 7]), vec![vec![1, 7]]);
}

// https://leetcode.cn/problems/insert-interval/description/

// 比较简单的思路是先插入，然后排序，最后合并（参见p56）
mod method1 {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals: Vec<Vec<i32>> = intervals;
        intervals.push(new_interval);
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans: Vec<Vec<i32>> = vec![];
        // 合并区间
        for interval in intervals {
            // 只要ans不为空，且interval和ans重叠，修改ans
            if !ans.is_empty() && ans.last().unwrap()[1] >= interval[0] {
                ans.last_mut().unwrap()[1] = ans.last().unwrap()[1].max(interval[1]);
            }
            // 否则，直接将interval添加到ans
            else {
                ans.push(interval);
            }
        }

        ans
    }
}

// intervals可以被分为三段：不重叠区域，重叠区域，不重叠区域
mod method2 {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let n: usize = intervals.len();
        let mut new_interval: Vec<i32> = new_interval;
        let mut i: usize = 0; // 指针
        let mut ans: Vec<Vec<i32>> = vec![];

        // 不重叠区域
        while i < n && intervals[i][1] < new_interval[0] {
            ans.push(intervals[i].clone());
            i += 1;
        }

        // 重叠区域，不好想到
        while i < n && intervals[i][0] <= new_interval[1] {
            new_interval[0] = new_interval[0].min(intervals[i][0]);
            new_interval[1] = new_interval[1].max(intervals[i][1]);
            i += 1;
        }
        ans.push(new_interval);

        // 不重叠区域
        while i < n {
            ans.push(intervals[i].clone());
            i += 1;
        }

        ans
    }
}
