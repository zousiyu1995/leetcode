from typing import Optional
from util.list_node import ListNode


class Solution:
    @staticmethod
    def hasCycle(head: Optional[ListNode]) -> bool:
        slow = head
        fast = head
        # 每次慢指针走1步，快指针走2步，如果有环，快指针肯定能追上慢指针
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
            if fast is slow:
                return True

        # 快指针追不上慢指针，肯定无环
        return False


def main():
    node1 = ListNode(3, None)
    node2 = ListNode(2, None)
    node3 = ListNode(0, None)
    node4 = ListNode(-4, None)
    node1.next = node2
    node2.next = node3
    node3.next = node4
    node4.next = node2

    print(Solution.hasCycle(node1))


if __name__ == "__main__":
    main()
