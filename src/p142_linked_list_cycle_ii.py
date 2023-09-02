from typing import Optional
from util.list_node import ListNode


class Solution:
    @staticmethod
    def detectCycle(head: Optional[ListNode]) -> Optional[ListNode]:
        # 本题实际上是个数学题
        slow = head
        fast = head
        # 每次慢指针走1步，快指针走2步
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
            # 快慢指针相遇，有环
            if fast is slow:
                while slow is not head:
                    slow = slow.next
                    head = head.next
                return slow

        # 快指针追不上慢指针，肯定无环
        return None


def main():
    node1 = ListNode(3, None)
    node2 = ListNode(2, None)
    node3 = ListNode(0, None)
    node4 = ListNode(-4, None)
    node1.next = node2
    node2.next = node3
    node3.next = node4
    node4.next = node2

    print(Solution.detectCycle(node1).val)


if __name__ == "__main__":
    main()
