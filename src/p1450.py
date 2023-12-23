import unittest
from typing import List


# 差分
class Solution1:
    @staticmethod
    def busyStudent(startTime: List[int], endTime: List[int], queryTime: int) -> int:
        from sortedcontainers import SortedDict

        n = len(startTime)
        diff = SortedDict()
        for i in range(n):
            diff[startTime[i]] = diff.get(startTime[i], 0) + 1
            diff[endTime[i] + 1] = diff.get(endTime[i] + 1, 0) - 1

        ans = 0
        for k in diff:
            if k <= queryTime:
                ans += diff[k]

        return ans


# 直接枚举也行，但是这题是个学差分的不错例子
class Solution2:
    @staticmethod
    def busyStudent(startTime: List[int], endTime: List[int], queryTime: int) -> int:
        ans = 0

        for i in range(len(startTime)):
            if queryTime >= startTime[i] and queryTime <= endTime[i]:
                ans += 1

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution2.busyStudent([1, 2, 3], [3, 2, 7], 4), 1)
        self.assertEqual(Solution2.busyStudent([4], [4], 4), 1)
        self.assertEqual(Solution2.busyStudent([4], [4], 5), 0)
        self.assertEqual(Solution2.busyStudent([1, 1, 1, 1], [1, 3, 2, 4], 7), 0)


if __name__ == "__main__":
    unittest.main()
