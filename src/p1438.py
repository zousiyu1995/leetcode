import unittest
from typing import List


# 只需要检查窗口的两个最值之间的绝对差是否小于等于limit即可
# 问题转换为如何维护窗口的最值
class Solution:
    @staticmethod
    def longestSubarray(nums: List[int], limit: int) -> int:
        from sortedcontainers import SortedList

        s_l = SortedList()  # sorted list
        ans = 0
        l = 0
        for r in range(len(nums)):
            s_l.add(nums[r])
            # 窗口不满足要求，移动窗口
            if s_l[-1] - s_l[0] > limit:
                s_l.remove(nums[l])
                l += 1
            ans = max(ans, r - l + 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.longestSubarray([8, 2, 4, 7], 4), 2)
        self.assertEqual(Solution.longestSubarray([10, 1, 2, 4, 7, 2], 5), 4)
        self.assertEqual(Solution.longestSubarray([4, 2, 2, 2, 4, 4, 2, 2], 0), 3)


if __name__ == "__main__":
    unittest.main()
