import unittest


# 哈希表
class Solution:
    @staticmethod
    def findTheDifference(s: str, t: str) -> str:
        from collections import Counter

        cnt = Counter()
        for c in s:
            cnt[c] += 1
        for c in t:
            cnt[c] -= 1
            if cnt[c] < 0:
                return c

        return ""


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.findTheDifference("abcd", "abcde"), "e")
        self.assertEqual(Solution.findTheDifference("a", "aa"), "a")


if __name__ == "__main__":
    unittest.main()
