import unittest
from typing import List

# https://leetcode.cn/problems/final-prices-with-a-special-discount-in-a-shop/description/


# 数据范围很小，暴力也可以
class Solution1:
    @staticmethod
    def finalPrices(prices: List[int]) -> List[int]:
        n = len(prices)
        ans = []

        for i in range(n):
            discount = 0
            for j in range(i + 1, n):
                if prices[j] <= prices[i]:
                    discount = prices[j]
                    break
            ans.append(prices[i] - discount)

        return ans


# 单调栈，维护最大栈，找下一个更小元素
class Solution2:
    @staticmethod
    def finalPrices(prices: List[int]) -> List[int]:
        n: int = len(prices)
        ans: List[int] = [0] * n
        stack: List[int] = []

        for i in range(n):
            # 只要栈不为空，且当前元素<=栈顶元素，说明栈顶元素遇到了下一个更小元素，此时折扣为prices[i]
            while stack and prices[i] <= prices[stack[-1]]:
                top: int = stack.pop()
                ans[top] = prices[top] - prices[i]
            # 否则，没有遇到下一个更小元素，此时没有折扣
            stack.append(i)
            ans[i] = prices[i]

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution2.finalPrices([8, 4, 6, 2, 3]), [4, 2, 4, 2, 3])
        self.assertEqual(Solution2.finalPrices([1, 2, 3, 4, 5]), [1, 2, 3, 4, 5])
        self.assertEqual(Solution2.finalPrices([10, 1, 1, 6]), [9, 0, 1, 6])


if __name__ == "__main__":
    unittest.main()
