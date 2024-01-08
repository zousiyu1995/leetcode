# https://leetcode.cn/problems/ransom-note/description/
import unittest


# 哈希表
class Solution:
    @staticmethod
    def canConstruct(ransomNote: str, magazine: str) -> bool:
        from collections import Counter

        cnt = Counter(magazine)
        for c in ransomNote:
            cnt[c] -= 1
            if cnt[c] < 0:  # magazine中相应的字符不够
                return False

        return True


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.canConstruct("a", "b"), False)
        self.assertEqual(Solution.canConstruct("aa", "ab"), False)
        self.assertEqual(Solution.canConstruct("aa", "aab"), True)


if __name__ == "__main__":
    unittest.main()
