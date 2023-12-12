import unittest
from typing import List


# 维护一个有序的窗口就行了
# 有序窗口使用有序序列（红黑树）实现
class Solution:
    @staticmethod
    def medianSlidingWindow(nums: List[int], k: int) -> List[float]:
        from sortedcontainers import SortedList

        ans = list()

        # 初始化第一个窗口
        window = SortedList(nums[:k])
        if k % 2 == 1:
            ans.append(window[k // 2])
        else:
            ans.append((window[k // 2] + window[k // 2 - 1]) / 2)

        # 遍历其他窗口
        for i in range(k, len(nums)):
            window.add(nums[i])
            window.remove(nums[i - k])
            if k % 2 == 1:
                ans.append(window[k // 2])
            else:
                ans.append((window[k // 2] + window[k // 2 - 1]) / 2)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution.medianSlidingWindow([1, 3, -1, -3, 5, 3, 6, 7], 3),
            [1, -1, -1, 3, 5, 6],
        )


if __name__ == "__main__":
    unittest.main()
