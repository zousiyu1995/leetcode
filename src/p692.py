# https://leetcode.cn/problems/top-k-frequent-words/description/

import unittest
from typing import List


# 哈希+排序
class Solution:
    @staticmethod
    def topKFrequent(words: List[str], k: int) -> List[str]:
        from collections import Counter

        cnt = Counter(words)
        # 双关键词排序
        # 第一个参数 -hash[word] 相当于词频的倒序排列。
        # 当词频相同时，用第二个参数 word 进行排序，即字母正序排列
        ans: List[str] = sorted(cnt, key=lambda word: (-cnt[word], word))

        return ans[:k]


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution.topKFrequent(["i", "love", "leetcode", "i", "love", "coding"], 2),
            ["i", "love"],
        )
        self.assertEqual(
            Solution.topKFrequent(
                ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"],
                4,
            ),
            ["the", "is", "sunny", "day"],
        )


if __name__ == "__main__":
    unittest.main()
