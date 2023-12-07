from typing import List
import unittest


# 滑移窗口+排序，暴力法，果断超时
class Solution1:
    @staticmethod
    def getSubarrayBeauty(nums: List[int], k: int, x: int) -> List[int]:
        n = len(nums)
        ans = []

        # 维护长度为k的窗口，同时维护第x小的数
        for i in range(0, n - k + 1):
            w = nums[i : i + k]
            w.sort()
            ans.append(min(0, w[x - 1]))

        return ans


# 本题的难点在于维护动态有序的滑动窗口，用特定数据结构解决
class Solution2:
    @staticmethod
    def getSubarrayBeauty(nums: List[int], k: int, x: int) -> List[int]:
        from sortedcontainers import SortedList

        ans = []
        w = SortedList(nums[0:k])
        ans.append(min(0, w[x - 1]))

        # 维护长度为k的窗口，同时维护第x小的数
        for i in range(1, len(nums) - k + 1):
            w.remove(nums[i - 1])
            w.add(nums[i + k - 1])
            ans.append(min(0, w[x - 1]))

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution2.getSubarrayBeauty([1, -1, -3, -2, 3], 3, 2), [-1, -2, -2]
        )
        self.assertEqual(
            Solution2.getSubarrayBeauty([-1, -2, -3, -4, -5], 2, 2), [-1, -2, -3, -4]
        )
        self.assertEqual(
            Solution2.getSubarrayBeauty([-3, 1, 2, -3, 0, -3], 2, 1),
            [-3, 0, -3, -3, -3],
        )


if __name__ == "__main__":
    unittest.main()
