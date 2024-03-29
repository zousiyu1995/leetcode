# My leetcode solutions

Written in `rust` and `python`.

❤️表示经典题目。

⭐表示难度，分为简单、中等和困难三个等级。

`()`表示备注。

- [My leetcode solutions](#my-leetcode-solutions)
  - [数组区间算法](#数组区间算法)
    - [前缀和](#前缀和)
    - [差分](#差分)
  - [二分查找](#二分查找)
  - [双指针](#双指针)
    - [滑动窗口](#滑动窗口)
    - [分组循环](#分组循环)
  - [字符串](#字符串)
  - [栈和队列](#栈和队列)
    - [单调栈](#单调栈)
  - [堆](#堆)
  - [哈希](#哈希)
  - [递归](#递归)
  - [链表](#链表)
  - [树](#树)
  - [分治](#分治)
  - [动态规划](#动态规划)
  - [贪心](#贪心)
  - [回溯](#回溯)
  - [数学](#数学)
    - [矩阵](#矩阵)
    - [位运算](#位运算)
  - [其他](#其他)

## 数组区间算法

### 前缀和

- [p42. 接雨水](./src/p42.rs)
- [p238. 除自身以外数组的乘积](./src/p238.rs)
- [p303. 区域和检索——数组不可变](./src/p303.rs)
- [p523. ❤️连续的子数组和](./src/p523.rs)
- [p525. ❤️连续数组](./src/p525.rs)
- [p560. ❤️和为K的子数组](./src/p560.rs)
- [p724. 寻找数组的中心下标](./src/p724.rs)
- [p930. ⭐⭐和相同的二元子数组](./src/p930.py) (?)
- [p1480. 一维数组的动态和](./src/p1480.rs)
- [p1685. 有序数组中差绝对值之和](./src/p1685.rs)
- [p1732. 找到最高海拔](./src/p1732.rs)
- [p1749. 任意子数组和的绝对值的最大值](./src/p1749.rs)

### 差分

- [p1094. ⭐⭐拼车](./src/p1094.py)
- [p1109. ❤️⭐⭐航班预订统计](./src/p1109.py)
- [p1450. ⭐在既定时间做作业的学生人数](./src/p1450.py)
- [p2381. ⭐⭐字母移位 II](./src/p2381.py)
- [p2406. ⭐⭐将区间分为最少组数](./src/p2406.py) (差分 || (贪心 && 堆))
- [p2772. ⭐⭐使数组中的所有元素都等于零](./src/p2772.py) (?)

## 二分查找

- [p33. ❤️搜索旋转排序数组](./src/p33.rs)
- [p34. ❤️在排序数组中查找元素的第一个和最后一个位置](./src/p34.rs)
- [p35. 搜索插入位置](./src/p35.rs)
- [p69. x的平方根](./src/p69.rs)
- [p81. ❤️搜索旋转排序数组 II](./src/p81.rs)
- [p153. 寻找旋转排序数组中的最小值](./src/p153.rs)
- [p162. 寻找峰值](./src/p162.rs)
- [p274. H指数](./src/p274.rs)
- [p275. H指数 II](./src/p275.rs)
- [p278. f第一个错误的版本](./src/p278.rs)
- [p367. 有效的完全平方数](./src/p367.rs)
- [p374. 猜数字大小](./src/p374.rs)
- [p540. 有序数组中的单一元素](./src/p540.rs)
- [p704. 二分查找](./src/p704.rs)
- [p852. 山脉数组的峰顶索引](./src/p852.rs)
- [p2300. 咒语和药水的成功对数](./src/p2300.rs)

## 双指针

- [p4. 寻找两个正序数组的中位数](./src/p4.rs)
- [p11. 盛最多水的容器](./src/p11.rs)
- [p15. 三数之和](./src/p15.rs)
- [p26. 删除有序数组中的重复项](./src/p26.rs)
- [p27. 移除元素](./src/p27.rs)
- [p56. ❤️合并区间](./src/p56.rs) (排序 && 区间问题)
- [p57. ❤️插入区间](./src/p57.rs) (排序 && 区间问题)
- [p80. 删除有序数组中的重复项 II](./src/p80.rs)
- [p88. 合并两个有序数组](./src/p88.rs)
- [p167. ❤️两数之和 II 输入有序数组](./src/p167.rs)
- [p283. 移动零](./src/p283.rs)
- [p392. 判断子序列](./src/p392.py)
- [p443. 压缩字符串](./src/p443.rs)
- [p633. 平方数之和](./src/p633.rs) (和p167类似)
- [p680. ❤️验证回文串 II](./src/p680.rs)
- [p713. 乘积小于K的子数组](./src/p713.rs)
- [p849. 到最近的人的最大距离](./src/p849.rs)
- [p977. 有序数组的平方](./src/p977.rs)
- [p986. ❤️区间列表的交集](./src/p986.rs)
- [p2461. 长度为K子数组中的最大和](./src/p2461.py)
- [p2562. 找出数组的串联值](./src/p2562.rs)
- [p2697. ⭐字典序最小回文串](./src/p2697.rs)
- [p2824. 统计和小于目标的下标对数目](./src/p2824.py)

### 滑动窗口

- [p3. ❤️⭐⭐无重复字符的最长子串](./src/p3.rs)
- [p30. ❤️串联所有单词的子串](./src/p30.py)
- [p76. ❤️最小覆盖子串](./src/p76.py)
- [p187. 重复的DNA序列](./src/p187.rs)
- [p209. 长度最小的子数组](./src/p209.rs)
- [p217. 存在重复元素](./src/p217.py)
- [p219. 存在重复元素 II](./src/p219.py)
- [p220. ❤️存在重复元素 III](./src/p220.py)
- [p295. ⭐⭐⭐数据流的中位数](./src/p295.py)
- [p424. 替换后的最长重复字符](./src/p424.py)
- [p438. 找到字符串中所有字母异位词](./src/p438.rs)
- [p480. ⭐⭐⭐滑动窗口中位数](./src/p480.py)
- [p567. 字符串的排列](./src/p567.py)
- [p594. 最长和谐子序列](./src/p594.py)
- [p643. 子数组最大平均数 I](./src/p643.rs)
- [p904. 水果成篮](./src/p904.py)
- [p992. ❤️⭐⭐⭐K个不同整数的子数组](./src/p992.rs)
- [p1004. 最大连续1的个数III](./src/p1004.py)
- [p1052. 爱生气的书店老板](./src/p1052.py)
- [p1208. 尽可能使字符串相等](./src/p1208.py)
- [p1234. ⭐⭐替换子串得到平衡字符串](./src/p1234.py)
- [p1343. 大小为K且平均值大于等于阈值的子数组数目](./src/p1343.py)
- [p1423. 可获得的最大点数](./src/p1423.rs)
- [p1438. 绝对差不超过限制的最长连续子数组](./src/p1438.py)
- [p1456. 定长子串中元音的最大数目](./src/p1456.rs)
- [p1493. 删掉一个元素以后全为1的最长子数组](./src/p1493.rs)
- [p1658. ⭐⭐将x减到0的最小操作数](./src/p1658.py)
- [p1695. ⭐⭐删除子数组的最大得分](./src/p1695.py)
- [p1838. 最高频元素的频数](./src/p1838.py)
- [p1984. 学生分数的最小差值](./src/p1984.py)
- [p2024. 考试的最大困扰度](./src/p2024.py)
- [p2090. 半径为k的子数组平均值](./src/p2090.py)
- [p2269. 找到一个数字的K美丽值](./src/p2269.rs)
- [p2379. 得到K个黑块的最少涂色次数](./src/p2379.py)
- [p2653. 滑动子数组的美丽值](./src/p2653.py)
- [p2841. 几乎唯一子数组的最大和](./src/p2841.py)
- [p2831. ⭐⭐找出最长等值子数组](./src/p2831.py) (?)
- [p2958. ⭐⭐最多K个重复元素的最长子数组](./src/p2958.py)
- [p2962. ❤️⭐⭐统计最大元素出现至少K次的子数组](./src/p2962.py) (求满足条件的区间的个数)

### 分组循环

- [p228. ⭐汇总区间](./src/p228.rs)
- [p1446. ⭐连续字符](./src/p1446.py)
- [p1578. ⭐⭐使绳子变成彩色的最短时间](./src/p1578.py)
- [p1759. ⭐⭐统计同质子字符串的数目](./src/p1759.py)
- [p1839. ⭐⭐所有元音按顺序排布的最长子字符串](./src/p1839.py)
- [p1869. ⭐哪种连续子字符串更长](./src/p1869.rs)
- [p1957. ⭐删除字符使字符串变好](./src/p1957.py)
- [p2038. ⭐⭐如果相邻两个颜色均相同则删除当前颜色](./src/p2038.py)
- [p2110. ⭐⭐股票平滑下跌阶段的数目](./src/p2110.py)
- [p2760. ⭐最长奇偶子数组](./src/p2760.py)

## 字符串

- [p5. 最长回文子串](./src/p5.rs) (中心扩散法)
- [p6. N字形变换](./src/p6.rs) (模拟 || 观察规律)
- [p8. 字符串转换整数（atoi）](./src/p8.rs)
- [p14. 最长公共前缀](./src/p14.rs)
- [p26. 找出字符串中第一个匹配项的下标](./src/p28.rs)
- [p28. 找出字符串中第一个匹配项的下标](./src/p28.rs)
- [p58. 最后一个单词的长度](./src/p58.py)
- [p125. 验证回文串](./src/p125.rs)
- [p151. 反转字符串中的单词](./src/p151.rs)
- [p205. 同构字符串](./src/p205.py) (哈希表双射)
- [p242. 有效的字母异位词](./src/p242.rs)
- [p344. 反转字符串](./src/p344.rs)
- [p345. 反转字符串中的元音字母](./src/p345.rs)
- [p347. 前k个高频元素](./src/p347.rs)
- [p459. 重复的子字符串](./src/p459.rs)
- [p541. 反转字符串 II](./src/p541.rs)
- [p1507. 转变日期格式](./src/p1507.rs)
- [p1657. 确定两个字符串是否接近](./src/p1657.rs)
- [p1768. 交替合并字符串](./src/p1768.rs)
- [p1876. 长度为三且各字符不同的子字符串](./src/p1876.rs)
- [p2011. 执行操作后的变量值](./src/p2011.rs)
- [p2337. 移动片段得到字符串](./src/p2337.rs)
- [p2586. 统计范围内的元音字符串数](./src/p2586.rs)
- [p2678. 老人的数目](./src/p2678.rs)
- [p2828. 判别首字母缩略词](./src/p2828.rs)

## 栈和队列

- [p20. 有效的括号](./src/p20.rs)
- [p32. 最长有效括号](./src/p32.rs)
- [p71. 简化路径](./src/p71.rs)
- [p150. 逆波兰表达式求值](./src/p150.rs)
- [p155. 最小栈](./src/p155.rs)
- [p225. 用队列实现栈](./src/p225.rs)
- [p232. 用栈实现队列](./src/p232.rs)
- [p239. 滑动窗口最大值](./src/p239.rs)
- [p394. ❤️字符串解码](./src/p394.rs)
- [p735. 小行星碰撞](./src/p735.rs)
- [p1047. 删除字符串中的所有相邻重复项](./src/p1047.rs)
- [p2216. 美化数组的最少删除数](./src/p2216.rs)
- [p2810. 故障键盘](./src/p2810.rs)
- [p2390. 从字符串中移除星号](./src/p2390.rs)

### 单调栈

- [p84. ⭐⭐⭐柱状图中最大的矩形](./src/p84.rs)
- [p316. ❤️⭐⭐去除重复字母](./src/p316.py) (字典序)
- [p402. ⭐⭐移掉K位数字](./src/p402.py) (字典序)
- [p496. ❤️⭐下一个更大元素 I](./src/p496.rs)
- [p503. ⭐⭐下一个更大元素 II](./src/p503.rs)
- [p739. ❤️⭐⭐每日温度](./src/p739.rs)
- [p901. ⭐⭐股票价格跨度](./src/p901.py)
- [p962. ⭐⭐最大宽度坡](./src/p962.py)
- [p1124. ⭐⭐表现良好的最长时间段](./src/p1124.py) (与p962类似)
- [p1475. ⭐商品折扣后的最终价格](./src/p1475.py)
- [p1944. ❤️⭐⭐⭐队列中可以看到的人数](./src/p1944.py)

## 堆

- [p215. 数组中的第K个最大元素](./src/p215.rs)
- [p264. 丑数 II](./src/p264.rs)
- [p1962. 移除石子使总数最小](./src/p1962.rs)
- [p2208. 将数组和减半的最少操作次数](./src/p2208.rs)
- [p2233. K次增加后的最大成绩](./src/p2233.rs)
- [p2530. 执行K次操作后的最大分数](./src/p2530.rs) (堆 && 贪心)
- [p2558. 从数量最多的堆取走礼物](./src/p2558.rs)

## 哈希

- [p1. ⭐两数之和](./src/p1.rs)
- [p13. 罗马数字转整数](./src/p13.rs)
- [p36. 有效的数独](./src/p36.rs)
- [p41. 缺失的第一个正数](./src/p41.rs)
- [p49. 字母异位词分组](./src/p49.rs)
- [p128. ❤️最长连续序列](./src/p128.rs)
- [p229. 多数元素 II](./src/p229.rs)
- [p268. 丢失的数字](./src/p268.rs)
- [p290. 单词规律](./src/p290.rs)
- [p349. 两个数组的交集](./src/p349.rs)
- [p350. ⭐两个数组的交集 II](./src/p350.py)
- [p383. ⭐赎金信](./src/p383.py)
- [p387. ⭐字符串中的第一个唯一字符](./src/p387.py)
- [p389. ⭐找不同](./src/p389.py)
- [p409. ⭐最长回文串](./src/p409.py)
- [p447. ⭐⭐回旋镖的数量](./src/p447.py)
- [p448. 找到所有数组中消失的数字](./src/p448.rs) (原地哈希)
- [p451. ⭐⭐根据字符出现频率排序](./src/p451.py) (哈希+排序)
- [p454. 四数相加 II](./src/p454.rs)
- [p692. ❤️⭐⭐前K个高频单词](./src/p692.py) (哈希+排序)
- [p1002. ⭐查找共用字符](./src/p1002.py)
- [p1207. 独一无二的出现次数](./src/p1207.rs)
- [p1267. 统计参与通信的服务器](./src/p1267.rs)
- [p1410. HTML实体解析器](./src/p1410.rs)
- [p1726. 同积元组](./src/p1726.rs)
- [p2103. 环和杆](./src/p2103.rs)
- [p2215. 找出两数组的不同](./src/p2215.rs)
- [p2273. 移除字母异位词后的结果数组](./src/p2273.rs)
- [p2341. 数组能形成多少数对](./src/p2341.rs)
- [p2342. 数位和相等数对的最大和](./src/p2342.rs)
- [p2367. 算术三元组的数目](./src/p2367.rs)
- [p2395. 和相等的子数组](./src/p2395.rs)
- [p2605. 从两个数字数组里生成最小数字](./src/p2605.rs)
- [p2682. 找出转圈游戏输家](./src/p2682.rs)
- [p2815. 数组中的最大数对和](./src/p2815.rs)
- [p2869. 收集元素的最少操作次数](./src/p2869.rs)

## 递归

## 链表

- [p2. ⭐⭐两数相加](./src/p2.py)
- [p19. 删除链表的倒数第N个结点](./src/p19.py) (双指针)
- [p21. 合并两个有序链表](./src/p21.py)
- [p82. 删除排序链表中的重复元素 II](./src/p82.py)
- [p83. 删除排序链表中的重复元素](./src/p83.py)
- [p92. 反转链表II](./src/p92.py)
- [p141. 环形链表](./src/p141.py) (快慢指针)
- [p142. 环形链表 II](./src/p142.py) (快慢指针)
- [p143. 重排链表](./src/p143.py) (快慢指针)
- [p160. 相交链表](./src/p160.py) (哈希 && 双指针)
- [p206. 反转链表](./src/p206.rs)
- [p237. 删除链表中的节点](./src/p237.py)
- [p234. 回文链表](./src/p234.py) (双指针)
- [p876. 链表的中间节点](./src/p876.py)

## 树

- [p104. 二叉树的最大深度](./src/p104.py) (二叉树 && 递归)
- [p1448. 统计二叉树中好节点的数目](./src/p1448.py) (二叉树 && 递归)
- [p2236. 判断根结点是否等于子结点之和](./src/p2236.py) (二叉树)

## 分治

- [p50. Pow(x, n)](./src/p50.rs)
- [p395. ❤️⭐⭐至少有K个重复字符的最长子串](./src/p395.py)

## 动态规划

- [p53. ❤️最大子数组和](./src/p53.rs)
- [p70. 爬楼梯](./src/p70.rs)
- [p121. 买卖股票的最佳时机](./src/p121.rs)
- [p509. 斐波那契数](./src/p509.rs)
- [p746. 使用最小花费爬楼梯](./src/p746.rs)

## 贪心

- [p12. 整数转罗马数字](./src/p12.rs) (贪心 && 哈希)
- [p122. 买卖股票的最佳时机 II](./src/p122.rs)
- [p135. 分发糖果](./src/p135.rs)
- [p406. ❤️根据身高重建队列](./src/p406.rs) (排序 && 贪心)
- [p435. ❤️无重叠区间](./src/p435.rs)
- [p452. ❤️用最少数量的箭引爆气球](./src/p452.rs)
- [p455. 分发饼干](./src/p455.rs)
- [p605. 种花问题](./src/p605.rs)
- [p665. ❤️非递减数列](./src/p665.rs)
- [p763. 划分字母区间](./src/p763.rs) (贪心 && 哈希)

## 回溯

- [p17. ❤️电话号码的字母组合](./src/p17.rs)

## 数学

- [p7. 整数反转](./src/p7.rs)
- [p9. 回文数](./src/p9.rs)
- [p66. ⭐加一](./src/p66.rs)
- [p67. ⭐二进制求和](./src/p67.rs)
- [p168. Excel表列名称](./src/p168.rs)
- [p189. 轮转数组](./src/p189.rs)
- [p202. 快乐数](./src/p202.rs)
- [p223. 矩形面积](./src/p223.rs)
- [p263. 丑数](./src/p263.rs)
- [p326. 3的幂](./src/p326.rs)
- [p461. 汉明距离](./src/p461.rs)
- [p1071. 字符串的最大公因子](./src/p1071.rs)
- [p1281. 整数的各位积和之差](./src/p1281.rs)
- [p1431. 拥有最多糖果的孩子](./src/p1431.rs)
- [p1689. 十-二进制数的最少数目](./src/p1689.rs)
- [p2178. 拆分成最多数目的正偶数之和](./src/p2178.rs) (数学 && 贪心)
- [p2235. 两整数相加](./src/p2235.rs)
- [p2520. 统计能整除数字的位数](./src/p2520.rs)
- [p2544. 交替数字和](./src/p2544.rs)
- [p2582. 递枕头](./src/p2582.rs)
- [p2652. 倍数求和](./src/p2652.rs)
- [p2656. K个元素的最大和](./src/p2656.rs)

### 矩阵

- [p48. 旋转图像](./src/p48.rs)
- [p59. 螺旋矩阵 II](./src/p59.rs)
- [p73. 矩阵置零](./src/p73.rs)
- [p74. 搜索二维矩阵](./src/p74.rs) (矩阵 && 二分搜索)
- [p240. 搜索二维矩阵 II](./src/p240.rs) (矩阵 && 二分搜索)
- [p1572. 矩阵对角线元素的和](./src/p1572.rs)
- [p2352. 相等行列对](./src/p2352.rs) (矩阵 && 哈希)
- [p2500. 删除每行中的最大值](./src/p2500.rs) (矩阵 && 排序)
- [p2545. 根据第K场考试的分数排序](./src/p2545.rs) (矩阵 && 排序)

### 位运算

- [p136. 只出现一次的数字](./src/p136.rs)
- [p137. 只出现一次的数字 II](./src/p137.rs)
- [p260. 只出现一次的数字 III](./src/p260.rs)
- [p318. 最大单词长度乘积](./src/p318.rs)
- [p1486. 数组异或操作](./src/p1486.rs)

## 其他

- [p485. 最大连续1的个数](./src/p485.py)
- [p1822. 数组元素积的符号](./src/p1822.rs) (遍历)
- [p2089. 找出数组排序后的目标下标](./src/p2089.rs) (排序 && 遍历)
- [p2240. 买钢笔和铅笔的方案数](./src/p2240.rs) (枚举)
- [p2303. 计算应缴税款总额](./src/p2303.rs) (模拟)
- [p2525. 根据规则将箱子分类](./src/p2525.rs) (模拟)
- [p2609. 最长平衡子字符串](./src/p2609.rs) (模拟)
