# https://leetcode.cn/problems/longest-palindrome/description/

import unittest


# 哈希
class Solution:
    @staticmethod
    def longestPalindrome(s: str) -> int:
        from collections import Counter

        cnt = Counter(s)
        ans = 0

        for v in cnt.values():
            ans += v // 2 * 2
            if ans % 2 == 0 and v % 2 == 1:
                ans += 1

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.longestPalindrome("abccccdd"), 7)
        self.assertEqual(Solution.longestPalindrome("a"), 1)
        self.assertEqual(Solution.longestPalindrome("aaaaaccc"), 7)


if __name__ == "__main__":
    unittest.main()
