from itertools import accumulate

a = [1, 2, 3, 4, 5]
n = len(a)

# 直接维护增量的差分数组
diff = [0] * (n + 1)
diff[0] += 1
diff[3] -= 1

for i, delta in enumerate(accumulate(diff[:-1])):
    a[i] += delta

print(a)  # [2, 3, 4, 4, 5]
