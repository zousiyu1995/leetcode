# https://leetcode.cn/problems/count-subarrays-where-max-element-appears-at-least-k-times/description/

import unittest
from typing import List


# 滑动窗口，求满足条件的区间的个数
class Solution:
    @staticmethod
    def countSubarrays(nums: List[int], k: int) -> int:
        max_ = max(nums)
        ans = 0
        cnt = 0  # 记录 max_ 出现的次数
        l = 0

        # 枚举右端点
        for r in range(len(nums)):
            if nums[r] == max_:  # 遇到最大值，cnt + 1
                cnt += 1
            # 如果 cnt = k，不断移动左端点直至 cnt < k
            # 此时，以 r 为右端点，[0, l] 中任一元素为左端点的子数组均满足条件
            while cnt == k:
                if nums[l] == max_:
                    cnt -= 1
                l += 1
            ans += l

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.countSubarrays([1, 3, 2, 3, 3], 2), 6)
        self.assertEqual(Solution.countSubarrays([1, 4, 2, 1], 3), 0)


if __name__ == "__main__":
    unittest.main()
