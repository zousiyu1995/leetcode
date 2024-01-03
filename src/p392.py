import unittest


# 针对s中的每个字符c，贪心匹配t中的字符
class Solution:
    @staticmethod
    def isSubsequence(s: str, t: str) -> bool:
        i = 0
        j = 0
        while i < len(s) and j < len(t):
            # 只有两个字符相等时移动i
            if s[i] == t[j]:
                i += 1
            # 其他所有情况都移动j
            j += 1

        return i == len(s)


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.isSubsequence("abc", "ahbgdc"), True)
        self.assertEqual(Solution.isSubsequence("axc", "ahbgdc"), False)


if __name__ == "__main__":
    unittest.main()
