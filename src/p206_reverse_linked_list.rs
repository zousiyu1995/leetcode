#[test]
fn test() {}

use super::util::list_node::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre: Option<Box<ListNode>> = None;
    let mut cur: Option<Box<ListNode>> = head;

    // None 1 -> 2 -> 3 -> 4
    //  ^   ^
    // pre  cur
    // 只要当前节点不为空
    while let Some(mut node) = cur {
        cur = node.next.take();
        node.next = pre;
        pre = Some(node);
    }

    pre
}
