from typing import Optional
from util.list_node import ListNode


# 将链表的值复制到数组，判断数组是否是回文的
class Solution1:
    def isPalindrome(head: Optional[ListNode]) -> bool:
        # 将链表的值复制到数组
        values = list()
        tmp = head
        while tmp is not None:
            values.append(tmp.val)
            tmp = tmp.next

        # 判断数组是否是回文的
        # l = 0
        # r = len(values) - 1
        # while l < r:
        #     if values[l] == values[r]:
        #         l += 1
        #         r -= 1
        #     else:
        #         return False

        # return True

        # 更简单的方法判断数组是否是回文的，比较原数组和翻转的数组
        return values == values[::-1]


# 翻转后一半的链表，判断是否是回文。过程比较长，不做实现
class Solution2:
    def isPalindrome(head: Optional[ListNode]) -> bool:
        pass


def test():
    l = ListNode(1, None)
    l.insert(2)
    l.insert(2)
    l.insert(1)

    print(Solution1.isPalindrome(l))


if __name__ == "__main__":
    test()
