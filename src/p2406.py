import unittest
from typing import List


# https://leetcode.cn/problems/divide-intervals-into-minimum-number-of-groups/description/
# 差分
# 转换成上下车模型，每个区间看成一个人，他在left时刻上车，right+1时刻下车，最后答案为同时在车上的人数的最大值
class Solution1:
    @staticmethod
    def minGroups(intervals: List[List[int]]) -> int:
        # 先找区间的最大值
        max_ = 0
        for _, r in intervals:
            max_ = max(max_, r)

        diff = [0] * (max_ + 1)
        for l, r in intervals:
            diff[(l - 1)] += 1
            diff[(r - 1) + 1] -= 1

        ans = 0
        sum_ = 0
        for i in diff:
            sum_ += i
            ans = max(ans, sum_)

        return ans


# 贪心+堆
class Solution2:
    @staticmethod
    def minGroups(intervals: List[List[int]]) -> int:
        from heapq import heappush, heapreplace

        # 有序才能保证贪心是有效的
        # 先排序，保证了左端点有序
        intervals.sort(key=lambda i: i[0])

        # 堆顶储存最后一个区间的right，分组时保证右端点有序
        heap = []
        for l, r in intervals:
            # 当前区间的left>最后区间的right，可以分为一组
            if heap and l > heap[0]:
                heapreplace(heap, r)
            # 否则，新建一个组
            else:
                heappush(heap, r)

        return len(heap)


class Test(unittest.TestCase):
    def test(self) -> None:
        self.assertEqual(
            Solution2.minGroups([[5, 10], [6, 8], [1, 5], [2, 3], [1, 10]]), 3
        )
        self.assertEqual(Solution2.minGroups([[1, 3], [5, 6], [8, 10], [11, 13]]), 1)


if __name__ == "__main__":
    unittest.main()
