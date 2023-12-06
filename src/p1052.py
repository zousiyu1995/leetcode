from typing import List
import unittest


# https://leetcode.cn/problems/grumpy-bookstore-owner/description/
class Solution:
    @staticmethod
    def maxSatisfied(customers: List[int], grumpy: List[int], minutes: int) -> int:
        n: int = len(customers)

        # 不生气的位置所对应的顾客永远都满意，这构成答案的第一部分
        ans1: int = 0
        for i in range(n):
            if grumpy[i] == 0:
                ans1 += customers[i]
                customers[i] = 0  # 这一步很关键

        # 在customers中找一段长度为minutes的、连续的、和最大的子数组，这构成答案的第二部分
        ans2: int = 0
        cnt: int = 0
        for i in range(n):
            cnt += customers[i]
            if i >= minutes:  # 移动窗口，维护窗口的和
                cnt -= customers[i - minutes]
            ans2 = max(ans2, cnt)

        return ans1 + ans2


class Test(unittest.TestCase):
    def test(self):
        self.assertEqual(
            Solution.maxSatisfied(
                [1, 0, 1, 2, 1, 1, 7, 5], [0, 1, 0, 1, 0, 1, 0, 1], 3
            ),
            16,
        )
        self.assertEqual(
            Solution.maxSatisfied([1], [0], 1),
            1,
        )


if __name__ == "__main__":
    unittest.main()
