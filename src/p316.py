# https://leetcode.cn/problems/remove-duplicate-letters/description/

import unittest


# 单调栈
class Solution:
    @staticmethod
    def removeDuplicateLetters(s: str) -> str:
        from collections import Counter

        stack: list[str] = []
        cnt = Counter(s)  # 各字符出现的次数
        seen = set()  # 用来判断字符是否出现过，判断是否在哈希表中只需要O(1)的时间，但判断是否在stack中需要O(N)的时间

        for c in s:
            if c not in seen:
                # 维护递增栈。只要当前字符的字典序<栈顶元素的字典序，说明用当前元素替换栈顶元素能得到更小的字典序
                while stack and c < stack[-1] and cnt[stack[-1]] > 0:
                    seen.discard(stack.pop())
                stack.append(c)
                seen.add(c)
            cnt[c] -= 1

        return "".join(stack)


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.removeDuplicateLetters("bcabc"), "abc")
        self.assertEqual(Solution.removeDuplicateLetters("cbacdcbc"), "acdb")


if __name__ == "__main__":
    unittest.main()
