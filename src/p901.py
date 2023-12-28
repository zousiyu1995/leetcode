import unittest


# 相当于，从后往前数，找下一个更大值
class StockSpanner:
    def __init__(self) -> None:
        from math import inf

        # 在栈中维护当前的天数和price
        self.stack: list[tuple[int, float]] = [(-1, inf)]
        self.cur_day = -1

    def next(self, price: int) -> int:
        # 维护最小栈
        # 移除栈中小于等于price的数字
        while price >= self.stack[-1][1]:
            self.stack.pop()
        self.cur_day += 1
        # 此时栈顶就是price的前一个更大元素
        ans = self.cur_day - self.stack[-1][0]
        self.stack.append((self.cur_day, price))

        return ans


class Test(unittest.TestCase):
    def test(self) -> None:
        ins = StockSpanner()
        ans = []
        stream = [100, 80, 60, 70, 60, 75, 85]
        for i in stream:
            ans.append(ins.next(i))
        self.assertEqual(ans, [1, 1, 1, 2, 1, 4, 6])


if __name__ == "__main__":
    unittest.main()
