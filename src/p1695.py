import unittest
from typing import List


# 题意，找一段最长且无重复的子数组，让其和最大。和第3题一样
class Solution:
    @staticmethod
    def maximumUniqueSubarray(nums: List[int]) -> int:
        ans = 0
        set_ = set()
        sum_ = 0
        l = 0
        for r in range(len(nums)):
            # 只要窗口不满足要求，移动窗口
            while nums[r] in set_:
                set_.remove(nums[l])
                sum_ -= nums[l]
                l += 1

            set_.add(nums[r])
            sum_ += nums[r]
            ans = max(ans, sum_)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.maximumUniqueSubarray([4, 2, 4, 5, 6]), 17)
        self.assertEqual(Solution.maximumUniqueSubarray([5, 2, 1, 2, 5, 2, 1, 2, 5]), 8)


if __name__ == "__main__":
    unittest.main()
