from typing import Optional
from util.list_node import ListNode


# 相当于找重复元素，应该第一时间想到哈希表
# 时间复杂度O(N)，空间复杂度O(N)
class Solution1:
    @staticmethod
    def getIntersectionNode(headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        # 遍历链表A，用集合储存已被访问过的节点
        visited = set()
        tmp = headA
        while tmp is not None:
            visited.add(tmp)
            tmp = tmp.next

        # 遍历链表B，如果出现在集合中，返回
        tmp = headB
        while tmp is not None:
            if tmp in visited:
                return tmp
            tmp = tmp.next

        # 遍历完了没发现，返回None
        return None


# 链表的思路不好想，无论是有环还是快慢指针，都不太好想
# 时间复杂度O(N)，空间复杂度O(1)
class Solution2:
    @staticmethod
    def getIntersectionNode(headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        # 只要有一个链表是None，不可能相交，返回None
        if headA is None or headB is None:
            return None

        a = headA
        b = headB
        while a is not b:
            a = headB if a is None else a.next
            b = headA if b is None else b.next

        return a


def test1():
    A1 = ListNode(4, None)
    A2 = ListNode(1, None)
    A3 = ListNode(8, None)
    A4 = ListNode(4, None)
    A5 = ListNode(5, None)
    A1.next = A2
    A2.next = A3
    A3.next = A4
    A4.next = A5

    B1 = ListNode(5, None)
    B2 = ListNode(6, None)
    B3 = ListNode(1, None)
    B1.next = B2
    B2.next = B3
    B3.next = A3

    print(Solution2.getIntersectionNode(A1, B1))


if __name__ == "__main__":
    test1()
