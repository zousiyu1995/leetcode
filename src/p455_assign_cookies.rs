#[test]
fn test() {
    // assert_eq!(find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
    // assert_eq!(find_content_children(vec![1, 2], vec![1, 2, 3]), 2);
    assert_eq!(
        find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8]),
        2
    );
}

// 贪心+排序
// g是胃口，其长度是孩子的数量
// s是饼干的尺寸，其长度是饼干的数量
pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut children: Vec<i32> = g;
    let mut cookies: Vec<i32> = s;
    children.sort();
    cookies.sort();

    let (mut child, mut cookie) = (0, 0);
    while child < children.len() && cookie < cookies.len() {
        // 如果饼干的尺寸满足孩子的胃口
        if children[child] <= cookies[cookie] {
            child += 1;
        }
        // 满足胃口，看下一块饼干
        // 不满足胃口，也要看下一块饼干
        cookie += 1;
    }

    child as i32
}
