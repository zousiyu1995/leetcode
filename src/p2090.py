import unittest
from typing import List


# https://leetcode.cn/problems/k-radius-subarray-averages/description/
class Solution:
    @staticmethod
    def getAverages(nums: List[int], k: int) -> List[int]:
        n = len(nums)

        # 前缀和
        presum = [0]
        for i in range(0, n):
            presum.append(presum[i] + nums[i])

        ans = [-1] * n
        size = 2 * k + 1  # 滑移窗口的大小

        # 窗口比n小，用滑移窗口生成ans；否则，直接返回ans
        if size <= n:
            # 窗口左端点取值范围[0, n - size]
            for i in range(0, n - size + 1):
                sum = presum[i + size] - presum[i]
                ans[i + k] = sum // size

        return ans


class Test(unittest.TestCase):
    def test(self):
        self.assertEqual(
            Solution.getAverages([7, 4, 3, 9, 1, 8, 5, 2, 6], 3),
            [-1, -1, -1, 5, 4, 4, -1, -1, -1],
        )
        self.assertEqual(
            Solution.getAverages([100000], 0),
            [1e5],
        )
        self.assertEqual(
            Solution.getAverages([8], 100000),
            [-1],
        )


if __name__ == "__main__":
    unittest.main()
