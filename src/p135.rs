#[test]
fn test() {
    assert_eq!(candy(vec![1, 0, 2]), 5);
    assert_eq!(candy(vec![1, 2, 2]), 4);
    assert_eq!(candy(vec![1, 2, 87, 87, 87, 2, 1]), 13);
}

// 不管如何，每个孩子先发一颗糖
// 相邻的孩子中，评分高的孩子必须获得更多的糖果
// 这句话可以拆分为两个规则：左边评分>右边评分；右边评分>左边评分
pub fn candy(ratings: Vec<i32>) -> i32 {
    let mut candy_num: Vec<i32> = vec![1; ratings.len()];

    // 从左到右遍历，如果右边的评分>左边的评分，右边的糖果=左边的糖果数+1
    for i in 1..ratings.len() {
        // i是右边的
        if ratings[i] > ratings[i - 1] {
            candy_num[i] = candy_num[i - 1] + 1;
        }
    }

    // 从右到左遍历，如果左边的评分>右边的评分，且左边的糖果<=右边的糖果，左边的糖果数=右边的糖果数+1
    for i in (0..ratings.len() - 1).rev() {
        // i是左边的
        if ratings[i] > ratings[i + 1] && candy_num[i] <= candy_num[i + 1] {
            candy_num[i] = candy_num[i + 1] + 1;
        }
    }

    candy_num.iter().sum()
}
