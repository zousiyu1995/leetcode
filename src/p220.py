import unittest
from typing import List


# 要求2：下标之差的绝对值不超过indexDiff，维护大小为indexDiff+1的窗口即可
# 要求3：值之差的绝对值不超过valueDiff，使用有序的窗口可以快速找到差值最小的两个元素
# 要求1：两个元素下标不一样，有序集合
class Solution:
    @staticmethod
    def containsNearbyAlmostDuplicate(
        nums: List[int], indexDiff: int, valueDiff: int
    ) -> bool:
        from sortedcontainers import SortedList

        window = SortedList()
        for i in range(len(nums)):
            window.add(nums[i])  # 无论如何，添加元素
            if i >= (indexDiff + 1):  # 窗口不满足要求，移除元素
                window.remove(nums[i - (indexDiff + 1)])

            # 二分查找，找与nums[i]相邻的元素的上界和下界
            idx = window.bisect_left(nums[i])
            # nums[i]左侧相邻元素的上界
            if idx > 0 and abs(window[idx] - window[idx - 1]) <= valueDiff:
                return True
            # nums[i]右侧相邻元素的下界
            if (
                idx < len(window) - 1
                and abs(window[idx + 1] - window[idx]) <= valueDiff
            ):
                return True

        return False


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution.containsNearbyAlmostDuplicate([1, 2, 3, 1], 3, 0), True
        )
        self.assertEqual(
            Solution.containsNearbyAlmostDuplicate([1, 5, 9, 1, 5, 9], 2, 3), False
        )


if __name__ == "__main__":
    unittest.main()
