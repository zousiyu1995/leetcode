# https://leetcode.cn/problems/add-two-numbers/description/

import unittest
from typing import Optional

from util.list_node import ListNode


# 链表，迭代
class Solution:
    @staticmethod
    def addTwoNumbers(
        l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:
        cur = dummy = ListNode()  # 哨兵节点
        carry = 0  # 进位
        while l1 or l2 or carry:  # 只要两个节点不全是空，或者还有进位
            carry += (l1.val if l1 else 0) + (l2.val if l2 else 0)
            cur.next = ListNode(carry % 10)  # 取余即是当前数位
            carry //= 10  # 商是新的进位
            cur = cur.next
            if l1:
                l1 = l1.next
            if l2:
                l2 = l2.next

        return dummy.next


class Test(unittest.TestCase):
    def test(self) -> None:
        l1 = ListNode(2)
        l1.insert(4)
        l1.insert(3)

        l2 = ListNode(5)
        l2.insert(6)
        l2.insert(4)

        print(Solution.addTwoNumbers(l1, l2))


if __name__ == "__main__":
    unittest.main()
