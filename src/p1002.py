# https://leetcode.cn/problems/find-common-characters/description/
import unittest
from typing import List


class Solution:
    @staticmethod
    def commonChars(words: List[str]) -> List[str]:
        from string import ascii_lowercase

        # 构建每个单词的字典
        # bella  -> [1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        # label  -> [1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        # roller -> [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0]
        # ans    ->              e                   l*2
        cnt: list[list[int]] = []
        for word in words:
            tmp: list[int] = [0] * 26
            for c in word:
                tmp[ord(c) - ord("a")] += 1
            cnt.append(tmp)

        ans: list[str] = []
        for j in range(26):
            # 矩阵第j列的最小值，即字符的重复次数
            rpt: int = len(words[0])
            for i in range(len(words)):
                rpt = min(rpt, cnt[i][j])

            # 添加答案，将第j个字符重复rpt次
            for _ in range(rpt):
                ans.append(ascii_lowercase[j])

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution.commonChars(["bella", "label", "roller"]), ["e", "l", "l"]
        )
        self.assertEqual(Solution.commonChars(["cool", "lock", "cook"]), ["c", "o"])


if __name__ == "__main__":
    unittest.main()
