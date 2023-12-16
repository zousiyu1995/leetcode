import unittest


# 根据删除规则，删除任意一个A不会影响可被删删除的B的数量，反之亦然。
# 因此直接统计「可删除的A的数量」和「可删除的B的数量」，比较两者的大小即可
class Solution:
    @staticmethod
    def winnerOfGame(colors: str) -> bool:
        cntA = 0
        cntB = 0
        i = 0
        while i < len(colors):
            start = i
            i += 1
            # 更新字符相同的区间
            while i < len(colors) and colors[i] == colors[i - 1]:
                i += 1
            # 更新可删除的A和B，实际上就是字符相同的区间的长度-2
            if colors[start] == "A":
                cntA += max(i - start - 2, 0)
            else:
                cntB += max(i - start - 2, 0)

        return cntA > cntB


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(Solution.winnerOfGame("AAABABB"), True)
        self.assertEqual(Solution.winnerOfGame("AA"), False)
        self.assertEqual(Solution.winnerOfGame("ABBBBBBBAAA"), False)


if __name__ == "__main__":
    unittest.main()
