import unittest
from itertools import accumulate
from typing import List


class Solution:
    @staticmethod
    def shiftingLetters(s: str, shifts: List[List[int]]) -> str:
        from string import ascii_lowercase

        n = len(s)
        diff = [0] * (n + 1)

        for start, end, direction in shifts:
            if direction == 1:  # 向后
                diff[start] += 1
                diff[end + 1] -= 1
            else:  # 向前
                diff[start] -= 1
                diff[end + 1] += 1

        ans = ""
        for c, shift in zip(s, accumulate(diff[:-1])):
            ans += ascii_lowercase[(ord(c) + shift - ord("a")) % 26]

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution.shiftingLetters("abc", [[0, 1, 0], [1, 2, 1], [0, 2, 1]]), "ace"
        )
        self.assertEqual(
            Solution.shiftingLetters("dztz", [[0, 0, 0], [1, 1, 1]]), "catz"
        )


if __name__ == "__main__":
    unittest.main()
