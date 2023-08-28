// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 自定义方法

// 从数组创建链表
// impl<const N: usize> From<[i32; N]> for ListNode {
//     // from [1, 2, 3, 4, 5]
//     //  to   1->2->3->4->5
//     fn from(array: [i32; N]) -> Self {}
// }
