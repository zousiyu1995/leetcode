import unittest
from typing import List


# 找出每一组平滑下降的子数组，其长度为n，其中平滑下降定义为后一天比前一天恰好小1
# 对于每个子数组，平滑下降阶段的数目为1+2+..+n = n*(1+n)/2
class Solution:
    @staticmethod
    def getDescentPeriods(prices: List[int]) -> int:
        ans = 0
        i = 0
        while i < len(prices):
            start = i
            i += 1
            while i < len(prices) and prices[i] - prices[i - 1] == -1:
                i += 1
            ans += (i - start) * (1 + (i - start)) // 2

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.getDescentPeriods([3, 2, 1, 4]), 7)
        self.assertEqual(Solution.getDescentPeriods([8, 6, 7, 7]), 4)
        self.assertEqual(Solution.getDescentPeriods([1]), 1)


if __name__ == "__main__":
    unittest.main()
