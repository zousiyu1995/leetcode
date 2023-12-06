import unittest


# https://leetcode.cn/problems/permutation-in-string/
# 如果s2包含s1的字符的排列之一，返回true，否则false
# 滑移窗口+哈希表，如果s2的窗口上的哈希表等于s1的哈希表，返回True，否则返回False
class Solution:
    @staticmethod
    def checkInclusion(s1: str, s2: str) -> bool:
        # 窗口长度
        n: int = len(s1)
        # 构建s1的哈希表
        map1: list[int] = [0] * 26
        for c in s1:
            map1[ord(c) - ord("a")] += 1

        # 在s2上应用滑移窗口
        map2: list[int] = [0] * 26
        for i, c in enumerate(s2):
            map2[ord(c) - ord("a")] += 1
            if i >= n:
                map2[ord(s2[i - n]) - ord("a")] -= 1
            if map2 == map1:
                return True

        return False


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.checkInclusion("ab", "eidbaooo"), True)
        self.assertEqual(Solution.checkInclusion("ab", "eidboaoo"), False)


if __name__ == "__main__":
    unittest.main()
