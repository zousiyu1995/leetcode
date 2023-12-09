#[test]
fn test() {
    assert_eq!(
        reconstruct_queue(
            vec![[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]
                .iter_mut()
                .map(|x| x.to_vec())
                .collect()
        ),
        vec![[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]]
            .iter_mut()
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<i32>>>()
    );

    assert_eq!(
        reconstruct_queue(
            vec![[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]]
                .iter_mut()
                .map(|x| x.to_vec())
                .collect()
        ),
        vec![[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]]
            .iter_mut()
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<i32>>>()
    );
}

// 整数对(h, k)，h表示这个人的身高，k表示这个人前面身高大于等于h的人数
// 按h降序排序后，每个人前面就是大于等于他身高的人
// 按k升序排序
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people: Vec<Vec<i32>> = people;
    people.sort_by(|a: &Vec<i32>, b: &Vec<i32>| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));

    let mut ans: Vec<Vec<i32>> = vec![];
    for p in people {
        ans.insert(p[1] as usize, p);
    }

    ans
}
