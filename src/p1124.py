# https://leetcode.cn/problems/longest-well-performing-interval/description/
import unittest
from typing import List


# 单调栈
class Solution:
    @staticmethod
    def longestWPI(hours: List[int]) -> int:
        n: int = len(hours)
        presum: list[int] = [0] * (n + 1)  # 前缀和
        stack: list[int] = [0]

        for i, hour in enumerate(hours, 1):
            presum[i] = presum[i - 1] + (1 if hour > 8 else -1)
            # 维护单调递减栈
            if presum[i] < presum[stack[-1]]:
                stack.append(i)

        ans: int = 0
        for j in range(n, 0, -1):
            # 当前前缀和>栈顶前缀和，表示劳累天数>不劳累天数
            while stack and presum[j] > presum[stack[-1]]:
                ans = max(ans, j - stack.pop())

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.longestWPI([9, 9, 6, 0, 6, 6, 9]), 3)
        self.assertEqual(Solution.longestWPI([6, 6, 6]), 0)


if __name__ == "__main__":
    unittest.main()
