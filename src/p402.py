# https://leetcode.cn/problems/remove-k-digits/description/

import unittest


# 单调栈，维护递增栈
class Solution:
    @staticmethod
    def removeKdigits(num: str, k: int) -> str:
        remain: int = len(num) - k
        stack: list[str] = []

        for digit in num:
            # 当前元素<栈顶，意味着用当前元素替换栈顶元素后，得到的数字更小
            while k > 0 and stack and digit < stack[-1]:
                stack.pop()
                k -= 1
            stack.append(digit)

        # 保留前remain位，并移除前导0
        ans: str = "".join(stack[:remain]).lstrip("0")

        return "0" if ans == "" else ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.removeKdigits("1432219", 3), "1219")
        self.assertEqual(Solution.removeKdigits("10200", 1), "200")
        self.assertEqual(Solution.removeKdigits("10", 2), "0")
        self.assertEqual(Solution.removeKdigits("9", 1), "0")
        self.assertEqual(Solution.removeKdigits("1173", 2), "11")


if __name__ == "__main__":
    unittest.main()
