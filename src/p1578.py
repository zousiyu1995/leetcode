import unittest
from typing import List


# 找字符相同的子串，删除其中对应数字最小的那些字符
class Solution:
    @staticmethod
    def minCost(colors: str, neededTime: List[int]) -> int:
        ans = 0
        i = 0
        while i < len(colors):
            # start = i
            sum_ = neededTime[i]  # [start, i)内元素之和
            max_ = neededTime[i]  # [start, i)内的最大元素
            i += 1
            while i < len(colors) and colors[i] == colors[i - 1]:
                sum_ += neededTime[i]
                max_ = max(max_, neededTime[i])
                i += 1

            # 移除neededTime中[start, i)范围内最大的元素，统计其他元素之和
            ans += sum_ - max_

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.minCost("abaac", [1, 2, 3, 4, 5]), 3)
        self.assertEqual(Solution.minCost("abc", [1, 2, 3]), 0)
        self.assertEqual(Solution.minCost("aabaa", [1, 2, 3, 4, 1]), 2)


if __name__ == "__main__":
    unittest.main()
