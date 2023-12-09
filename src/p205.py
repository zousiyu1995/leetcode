import unittest


# 哈希表双射
class Solution:
    @staticmethod
    def isIsomorphic(s: str, t: str) -> bool:
        d1 = dict()
        d2 = dict()
        for c1, c2 in zip(s, t):
            if c1 in d1 and d1[c1] != c2 or c2 in d2 and d2[c2] != c1:
                return False
            d1[c1] = c2
            d2[c2] = c1

        return True


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.isIsomorphic("egg", "add"), True)
        self.assertEqual(Solution.isIsomorphic("foo", "bar"), False)
        self.assertEqual(Solution.isIsomorphic("paper", "title"), True)
        self.assertEqual(Solution.isIsomorphic("badc", "baba"), False)


if __name__ == "__main__":
    unittest.main()
