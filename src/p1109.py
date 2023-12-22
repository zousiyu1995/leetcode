import unittest
from typing import List


# https://leetcode.cn/problems/corporate-flight-bookings/
# 差分
class Solution:
    @staticmethod
    def corpFlightBookings(bookings: List[List[int]], n: int) -> List[int]:
        diff = [0] * (n + 1)
        # [first, last + 1)
        for first, last, seat in bookings:
            diff[(first - 1)] += seat
            diff[(last - 1) + 1] -= seat

        ans = [0] * n
        ans[0] = diff[0]
        for i in range(1, n):
            ans[i] = ans[i - 1] + diff[i]

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution.corpFlightBookings([[1, 2, 10], [2, 3, 20], [2, 5, 25]], 5),
            [10, 55, 45, 25, 25],
        )
        self.assertEqual(
            Solution.corpFlightBookings([[1, 2, 10], [2, 2, 15]], 2), [10, 25]
        )


if __name__ == "__main__":
    unittest.main()
