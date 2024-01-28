# https://leetcode.cn/problems/sort-characters-by-frequency/description/

import unittest


class Solution:
    @staticmethod
    def frequencySort(s: str) -> str:
        from collections import Counter

        cnt = Counter(s)
        cnt = dict(sorted(cnt.items(), key=lambda item: item[1], reverse=True))
        ans = ""

        for k, v in cnt.items():
            ans += k * v

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.frequencySort("tree"), "eert")
        self.assertEqual(Solution.frequencySort("cccaaa"), "cccaaa")
        self.assertEqual(Solution.frequencySort("Aabb"), "bbAa")


if __name__ == "__main__":
    unittest.main()
