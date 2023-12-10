import unittest
from typing import List


# 维护窗口，保证窗口满足条件
# 数组内元素以偶数开头，偶数奇数交错排列，数组内任意元素都小于threshold
class Solution:
    @staticmethod
    def longestAlternatingSubarray(nums: List[int], threshold: int) -> int:
        n = len(nums)
        ans = 0
        i = 0
        while i < n:
            # 保证开始元素满足要求
            if nums[i] > threshold or nums[i] % 2 != 0:
                i += 1
                continue
            start = i
            # 判断下一个元素是否满足要求
            i += 1
            while i < n and nums[i] <= threshold and nums[i] % 2 != nums[i - 1] % 2:
                i += 1
            # 从start到i-1是满足题意的子数组
            ans = max(ans, i - start)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.longestAlternatingSubarray([3, 2, 5, 4], 5), 3)
        self.assertEqual(Solution.longestAlternatingSubarray([1, 2], 2), 1)
        self.assertEqual(Solution.longestAlternatingSubarray([2, 3, 4, 5], 4), 3)


if __name__ == "__main__":
    unittest.main()
