from typing import Optional
from util.list_node import ListNode


def reverseBetween(
    head: Optional[ListNode], left: int, right: int
) -> Optional[ListNode]:
    p0 = dummy = ListNode(next=head)
    for _ in range(left - 1):
        p0 = p0.next

    pre = None
    cur = p0.next
    for _ in range(right - left + 1):
        nxt = cur.next
        cur.next = pre
        pre = cur
        cur = nxt

    p0.next.next = cur
    p0.next = pre
    return dummy.next


def main():
    head = ListNode(1, None)
    head.insert(2)
    head.insert(3)
    head.insert(4)
    head.insert(5)
    print(head)

    rev_head = reverseBetween(head, 2, 4)
    print(rev_head)


if __name__ == "__main__":
    main()
