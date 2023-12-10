import unittest
from typing import List


# 维护窗口，保证窗口的花销<=最大花销
class Solution:
    @staticmethod
    def equalSubstring(s: str, t: str, maxCost: int) -> int:
        ans = 0

        l = 0
        cost = 0
        for r in range(len(s)):
            cost += abs(ord(s[r]) - ord(t[r]))
            while cost > maxCost:
                cost -= abs(ord(s[l]) - ord(t[l]))
                l += 1
            ans = max(ans, r - l + 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.equalSubstring("abcd", "bcdf", 3), 3)
        self.assertEqual(Solution.equalSubstring("abcd", "cdef", 3), 1)
        self.assertEqual(Solution.equalSubstring("abcd", "acde", 0), 1)


if __name__ == "__main__":
    unittest.main()
