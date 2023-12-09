import unittest
from typing import List


class Solution:
    @staticmethod
    def findLHS(nums: List[int]) -> int:
        nums.sort()

        ans = 0
        l = 0
        for r in range(len(nums)):
            # 如果两个最值差别大于1，移动窗口
            while nums[r] - nums[l] > 1:
                l += 1
            # 如果两个最值差别正好是1，更新ans
            if nums[r] - nums[l] == 1:
                ans = max(ans, r - l + 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.findLHS([1, 3, 2, 2, 5, 2, 3, 7]), 5)
        self.assertEqual(Solution.findLHS([1, 2, 3, 4]), 2)
        self.assertEqual(Solution.findLHS([1, 1, 1, 1]), 0)


if __name__ == "__main__":
    unittest.main()
