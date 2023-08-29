#[test]
fn test() {
    let head: ListNode = ListNode::from([1, 2, 3, 4, 5]);
    head.print();

    let rev_head: Option<Box<ListNode>> = reverse_list(Some(Box::new(head)));
    rev_head.unwrap().print();
}

use super::util::list_node::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre: Option<Box<ListNode>> = None;
    let mut cur: Option<Box<ListNode>> = head;

    // None 1 -> 2 -> 3 -> 4 -> 5
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
