# 有序序列实现
class MedianFinder:
    def __init__(self) -> None:
        from sortedcontainers import SortedList

        self.window = SortedList()

    def addNum(self, num: int) -> None:
        self.window.add(num)

    def findMedian(self) -> float:
        n = len(self.window)
        if n % 2 == 1:
            return self.window[n // 2]
        else:
            return (self.window[n // 2] + self.window[n // 2 - 1]) / 2


# 一般是用堆来实现，有时间试试


def test() -> None:
    m = MedianFinder()
    m.addNum(1)
    m.addNum(2)
    print(m.findMedian())
    m.addNum(3)
    print(m.findMedian())


if __name__ == "__main__":
    test()
