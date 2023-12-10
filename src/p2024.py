import unittest
from typing import List


# 维护窗口，保证窗口的大小 <= 最多的那个字符的个数 + k
class Solution:
    @staticmethod
    def maxConsecutiveAnswers(answerKey: str, k: int) -> int:
        ans = 0
        cnt = dict()
        l = 0
        for r in range(len(answerKey)):
            cnt[answerKey[r]] = cnt.get(answerKey[r], 0) + 1
            # 窗口不满足要求
            while (r - l + 1) > max(cnt.values()) + k:
                cnt[answerKey[l]] -= 1
                l += 1

            ans = max(ans, r - l + 1)

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        # self.assertEqual(Solution.maxConsecutiveAnswers("TTFF", 2), 4)
        self.assertEqual(Solution.maxConsecutiveAnswers("TFFT", 1), 3)
        # self.assertEqual(Solution.maxConsecutiveAnswers("TTFTTFTT", 1), 5)
        # self.assertEqual(Solution.maxConsecutiveAnswers("FFTFTTTFFF", 1), 5)


if __name__ == "__main__":
    unittest.main()
