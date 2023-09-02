from typing import Optional
from util.list_node import ListNode


class Solution:
    @staticmethod
    def middleNode(head: Optional[ListNode]) -> Optional[ListNode]:
        slow = head
        fast = head
        # 每次慢指针走1步，快指针走2步，最终慢指针就是答案
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next

        return slow


def main():
    head = ListNode(1, None)
    head.insert(2)
    head.insert(3)
    head.insert(4)
    head.insert(5)
    print(head)

    mid_node = Solution.middleNode(head)
    print(mid_node)


if __name__ == "__main__":
    main()
