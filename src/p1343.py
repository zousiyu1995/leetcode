from typing import List
import unittest


# https://leetcode.cn/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/description/
class Solution:
    def numOfSubarrays(arr: List[int], k: int, threshold: int) -> int:
        # 前缀和
        presum = [0]
        for i in range(0, len(arr)):
            presum.append(presum[i] + arr[i])

        ans = 0
        # 维护长度为k滑移窗口，i的范围是[0, n - k]
        for i in range(0, len(arr) - k + 1):
            # 窗口的和
            sum = presum[i + k] - presum[i]
            if sum / k >= threshold:
                ans += 1

        return ans


class Test(unittest.TestCase):
    def test(self):
        self.assertEqual(Solution.numOfSubarrays([2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3)
        self.assertEqual(
            Solution.numOfSubarrays([11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5), 6
        )


if __name__ == "__main__":
    unittest.main()
