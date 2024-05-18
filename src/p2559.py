import unittest
from typing import List


# https://leetcode.cn/problems/count-vowel-strings-in-ranges/description/
class Solution:
    @staticmethod
    def vowelStrings(words: List[str], queries: List[List[int]]) -> List[int]:
        # 前缀和
        n: int = len(words)
        presum: List[int] = [0] * (n + 1)
        # [1, n]
        for i in range(1, n + 1):
            # 检查第n-1个元素是否以元音开头和结尾
            if words[i - 1][0] in "aeiou" and words[i - 1][-1] in "aeiou":
                presum[i] = presum[i - 1] + 1
            else:
                presum[i] = presum[i - 1] + 0

        ans: List[int] = []
        for l, r in queries:
            ans.append(presum[r + 1] - presum[l])

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution.vowelStrings(
                words=["aba", "bcb", "ece", "aa", "e"], queries=[[0, 2], [1, 4], [1, 1]]
            ),
            [2, 3, 0],
        )


if __name__ == "__main__":
    unittest.main()
