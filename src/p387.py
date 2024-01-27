# https://leetcode.cn/problems/first-unique-character-in-a-string/description/

import unittest


# 哈希
class Solution:
    @staticmethod
    def firstUniqChar(s: str) -> int:
        from collections import Counter

        cnt = Counter(s)

        for i, c in enumerate(s):
            if cnt[c] == 1:
                return i

        return -1


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.firstUniqChar("leetcode"), 0)
        self.assertEqual(Solution.firstUniqChar("loveleetcode"), 2)
        self.assertEqual(Solution.firstUniqChar("aabb"), -1)


if __name__ == "__main__":
    unittest.main()
