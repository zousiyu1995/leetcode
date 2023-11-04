# My leetcode solutions

Written in `rust`.

基本算法思想：枚举、模拟、剪枝、递归、分治、动态规划、贪心、回溯。

- [My leetcode solutions](#my-leetcode-solutions)
  - [二分查找](#二分查找)
  - [双指针](#双指针)
  - [前缀和、后缀和](#前缀和后缀和)
  - [字符串](#字符串)
  - [栈、队列](#栈队列)
  - [堆](#堆)
    - [单调栈](#单调栈)
  - [哈希表、哈希集](#哈希表哈希集)
  - [链表](#链表)
  - [数学](#数学)
  - [递归](#递归)
  - [树](#树)
  - [分治](#分治)
  - [动态规划](#动态规划)
  - [贪心](#贪心)
  - [回溯](#回溯)
  - [其他](#其他)

## 二分查找

- [p33. search in rotated sorted array 搜索旋转排序数组](./src/p33_search_in_rotated_sorted_array.rs) (经典题目)
- [p34. find first and last position of element in sorted array 在排序数组中查找元素的第一个和最后一个位置](./src/p34_find_first_and_last_position_of_element_in_sorted_array.rs) (二分查找经典题)
- [p35. search insert position 搜索插入位置](./src/p35_search_insert.rs)
- [p69. sqrtx x的平方根](./src/p69_my_sqrt.rs)
- [p153. find minimum in rotated sorted array 寻找旋转排序数组中的最小值](./src/p153_find_minimum_in_rotated_sorted_array.rs)
- [p162. find peak element 寻找峰值](./src/p162_find_peak_element.rs)
- [p274. h index H指数](./src/p274_h_index.rs)
- [p275. h index ii H指数 II](./src/p275_h_index_ii.rs)
- [p278. first bad version 第一个错误的版本](./src/p278_first_bad_version.rs)
- [p367. valid perfect square 有效的完全平方数](./src/p367_valid_perfect_square.rs)
- [p374. guss number higher or lower 猜数字大小](./src/p374_guess_number_higher_or_lower.rs)
- [p704. binary search 二分查找](./src/p704_binary_search.rs)
- [p852. peak index in a mountain array 山脉数组的峰顶索引](./src/p852_peak_index_in_a_mountain_array.rs)

## 双指针

- [p4. median of two sorted arrays 寻找两个正序数组的中位数](./src/p4_median_of_two_sorted_arrays.rs)
- [p11. containter with most water 盛最多水的容器](./src/p11_container_with_most_water.rs)
- [p15. 3sum 三数之和](./src/p15_3sum.rs)
- [p26. remove duplicates from sorted array 删除有序数组中的重复项](./src/p26_remove_duplicates_from_sorted_array.rs)
- [p27. remove element 移除元素](./src/p27_reomove_element.rs)
- [p56. merge intervals 合并区间](./src/p56_merge_intervals.rs) (排序+双指针)
- [p88. merge sorted array 合并两个有序数组](./src/p88_merge_sorted_array.rs)
- [p167. two sum ii input array is sorted 两数之和 II 输入有序数组](./src/p167_two_sum_ii_input_array_is_sorted.rs)
- [p209. minimum size subarray sum 长度最小的子数组](./src/p209_minimum_size_subarray_sum.rs) (滑移窗口)
- [p228. summary ranges 汇总区间](./src/p228_summary_ranges.rs) (滑移窗口)
- [p283. move zeros 移动零](./src/p283_move_zeroes.rs)
- [p443. string compression 压缩字符串](./src/p443_string_compression.rs) (双指针+字符串)
- [p643. maximum average subarray i 子数组最大平均数 I](./src/p643_maximum_average_subarray_i.rs) (滑移窗口)
- [p713. subarray product less than k 乘积小于K的子数组](./src/p713_subarray_product_less_than_k.rs)
- [p849. maximize distance to closest person 到最近的人的最大距离](./src/p849_maximize_distance_to_closest_person.rs)
- [p977. squares of a sorted array 有序数组的平方](./src/p977_squares_of_a_sorted_array.rs)
- [p2562. find the array concatenation value 找出数组的串联值](./src/p2562_find_the_array_concatenation_value.rs) (双指针+模拟)

## 前缀和、后缀和

- [p42. trapping rain water 接雨水](./src/p42_trapping_rain_water.rs) (前缀最大值、后缀最大值)
- [p238. product of array except self 除自身以外数组的乘积](./src/p238_product_of_array_except_self.rs)
- [p303. range sum query immutable 区域和检索——数组不可变](./src/p303_range_sum_query_immutable.rs)
- [p523. continuous subarray sum 连续的子数组和](./src/p523_continuous_subarray_sum.rs) (前缀和+哈希表，经典题)
- [p525. contiguous array 连续数组](./src/p525_contiguous_array.rs) (前缀和+哈希表，经典题)
- [p560. subarray sum equals k 和为K的子数组](./src/p560_subarray_sum_equals_k.rs) (前缀和+哈希表，经典题)
- [p724. find pivot index 寻找数组的中心下标](./src/p724_find_pivot_index.rs)
- [p1480. running sum of 1d array 一维数组的动态和](./src/p1480_running_sum_of_1d_array.rs) (前缀和)
- [p1685. sum of absolute differences in a sorted array 有序数组中差绝对值之和](./src/p1685_sum_of_absolute_differences_in_a_sorted_array.rs) (前缀和+后缀和)
- [p1732. find the highest altitude 找到最高海拔](./src/p1732_find_the_highest_altitude.rs)
- [p1749. maximum absolute sum of any subarray 任意子数组和的绝对值的最大值](./src/p1749_maximum_absolute_sum_of_any_subarray.rs)

## 字符串

- [p5. longest palindromic substring 最长回文子串](./src/p5_longest_palindromic_substring.rs) (中心扩散法)
- [p6. zigzag conversion N字形变换](./src/p6_zigzag_conversion.rs) (模拟 || 观察规律)
- [p8. string to integer atoi 字符串转换整数（atoi）](./src/p8_string_to_integer_atoi.rs)
- [p14. longest common prefix 最长公共前缀](./src/p14_longest_common_prefix.rs)
- [p26. find the index of the first occurrence in a string 找出字符串中第一个匹配项的下标](./src/p28_find_the_index_of_the_first_occurrence_in_a_string.rs)
- [p28. find the index of the first occurrence in a string 找出字符串中第一个匹配项的下标](./src/p28_find_the_index_of_the_first_occurrence_in_a_string.rs)
- [p125. valid palindrome 验证回文串](./src/p125_valid_palindrome.rs)
- [p151. reverse words in a string 反转字符串中的单词](./src/p151_reverse_words_in_a_string.rs)
- [p187. repeated dna sequence 重复的DNA序列](./src/p187_repeated_dna_sequences.rs) (哈希表+滑移窗口)
- [p344. reverse string 反转字符串](./src/p344_reverse_string.rs)
- [p345. reverse vowels of a string 反转字符串中的元音字母](./src/p345_reverse_vowels_of_a_string.rs)
- [p347. top k frequent elements 前k个高频元素](./src/p347_top_k_frequent_elements.rs)
- [p459. repeated substring pattern 重复的子字符串](./src/p459_repeated_substring_pattern.rs)
- [p541. reverse string ii 反转字符串 II](./src/p541_reverse_string_ii.rs)
- [p1507. reformat data 转变日期格式](./src/p1507_reformat_data.rs)
- [p1657. determine if two strings are close 确定两个字符串是否接近](./src/p1657_determine_if_two_strings_are_close.rs) (哈希表)
- [p1768. merge strings alternately 交替合并字符串](./src/p1768_merge_strings_alternately.rs)
- [p1869. longer contiguous segments of ones than zeros 哪种连续子字符串更长](./src/p1869_longer_contiguous_segments_of_ones_than_zeros.rs)
- [p1876. substrings of size three with distinct characters 长度为三且各字符不同的子字符串](./src/p1876_substrings_of_size_three_with_distinct_characters.rs)
- [p2011. final value of variable after performing operations 执行操作后的变量值](./src/p2011_final_value_of_variable_after_performing_operations.rs) (模拟)
- [p2337. move pieces to obtain a string 移动片段得到字符串](./src/p2337_move_pieces_to_obtain_a_string.rs)
- [p2678. number of senior citizens 老人的数目](./src/p2678_number_of_senior_citizens.rs) (字符串parse)
- [p2828. check if a string is an acronym of words 判别首字母缩略词](./src/p2828_check_if_a_string_is_an_acronym_of_words.rs)

## 栈、队列

- [p20. valid parentheses 有效的括号](./src/p20_valid_parentheses.rs)
- [p32. longest valid parentheses 最长有效括号](./src/p32_longest_valid_parentheses.rs)
- [p71. simplify path 简化路径](./src/p71_simplify_path.rs) (栈+字符串)
- [p150. evaluate reverse polish notation 逆波兰表达式求值](./src/p150_evaluate_reverse_polish_notation.rs)
- [p155. min stack 最小栈](./src/p155_min_stack.rs)
- [p225. implement stack using queues 用队列实现栈](./src/p225_implement_stack_using_queues.rs)
- [p232. implement queue using stacks 用栈实现队列](./src/p232_implement_queue_using_stacks.rs)
- [p239. sliding window maximum 滑动窗口最大值](./src/p239_sliding_window_maximum.rs)
- [p394. decode string 字符串解码](./src/p394_decode_string.rs) (栈，经典题)
- [p1047. remove all adjacent duplicates in string 删除字符串中的所有相邻重复项](./src/p1047_remove_all_adjacent_duplicates_in_string.rs)
- [p2810. faulty keyboard 故障键盘](./src/p2810_faulty_keyboard.rs)
- [p2390. removing stars from a string 从字符串中移除星号](./src/p2390_removing_stars_from_a_string.rs)

## 堆

优先队列一般使用堆来实现。

- [p215. kth-largest-element-in-an-array 数组中的第K个最大元素](./src/p215_kth_largest_element_in_an_array.rs) (大顶堆)
- [p1962. remove stones to minimize the total 移除石子使总数最小](./src/p1962_remove_stones_to_minimize_the_total.rs) (大顶堆)
- [p2208. minimum operations to halve array sum 将数组和减半的最少操作次数](./src/p2208_minimum_operations_to_halve_array_sum.rs) (大顶堆)
- [p2233. maximum product after k increments K次增加后的最大成绩](./src/p2233_maximum_product_after_k_increments.rs) (小顶堆)
- [p2530. maximal score after applying k operations 执行 K 次操作后的最大分数](./src/p2530_maximal_score_after_applying_k_operations.rs) (贪心+大顶堆)
- [p2558. take gifts from the richest pile 从数量最多的堆取走礼物](./src/p2558_take_gifts_from_the_richest_pile.rs) (大顶堆)

### 单调栈

单调栈用途不太广泛，只处理一种典型的问题，叫做Next Greater Element。

- [p84. largest rectangle in histogram 柱状图中最大的矩形](./src/p84_largest_rectangle_in_histogram.rs) (单调栈)
- [p496. next greater element 下一个更大元素 I](./src/p496_next_greater_element_i.rs) (单调栈+哈希表)
- [p503. next greater element ii 下一个更大元素 II](./src/p503_next_greater_element_ii.rs) (单调栈)
- [p739. daily temperatures 每日温度](./src/p739_daily_temperatures.rs) (单调栈)

## 哈希表、哈希集

- [p1. two sum 两数之和](./src/p1_two_sum.rs)
- [p3. longest substring without repeating characters 无重复字符的最长子串](./src/p3_longest_substring_without_repeating_characters.rs)
- [p13. roman to integer 罗马数字转整数](./src/p13_roman_to_integer.rs)
- [p36. valid sudoku 有效的数独](./src/p36_valid_sudoku.rs)
- [p41. first missing positive 缺失的第一个正数](./src/p41_first_missing_positive.rs) (哈希表)
- [p49. group anagrams 字母异位词分组](./src/p49_group_anagrams.rs)
- [p136. single number 只出现一次的数字](./src/p136_single_number.rs) (哈希表、异或)
- [p137. single number ii 只出现一次的数字 II](./src/p137_single_number_ii.rs) (哈希表、位运算)
- [p242. valid anagram 有效的字母异位词](./src/p242_valid_anagram.rs)
- [p260. single number iii 只出现一次的数字 III](./src/p260_single_number_iii.rs) (哈希表、位运算)
- [p268. missing number 丢失的数字](./src/p268_missing_number.rs) (数学、哈希集)
- [p290. word pattern 单词规律](./src/p290_word_pattern.rs)
- [p349. intersection of two array 两个数组的交集](./src/p349_intersection_of_two_arrays.rs)
- [p454. 4 sum ii 四数相加 II](./src/p454_4sum_ii.rs)
- [p1207. unique number of occurrences 独一无二的出现次数](./src/p1207_unique_number_of_occurrences.rs)
- [p1267. count servers that communicate 统计参与通信的服务器](./src/p1267_count_servers_that_communicate.rs)
- [p1726. tuple with same product 同积元组](./src/p1726_tuple_with_same_product.rs) (哈希表+组合数学)
- [p2103. rings and rods 环和杆](./src/p2103_rings_and_rods.rs) (哈希表+字符串)
- [p2215. find the difference of two arrays 找出两数组的不同](./src/p2215_find_the_difference_of_two_arrays.rs)
- [p2273. find resultant array after removing anagrams 移除字母异位词后的结果数组](./src/p2273_find_resultant_array_after_removing_anagrams.rs)
- [p2341. maximum number of pairs in array 数组能形成多少数对](./src/p2341_maximum_number_of_pairs_in_array.rs)
- [p2367. number of arithmetic triplets 算术三元组的数目](./src/p2367_number_of_arithmetic_triplets.rs)
- [p2395. find subarrays with equal sum 和相等的子数组](./src/p2395_find_subarrays_with_equal_sum.rs)
- [p2605. form smallest number from two digit arrays 从两个数字数组里生成最小数字](./src/p2605_form_smallest_number_from_two_digit_arrays.rs)
- [p2682. find the losers of the circular game 找出转圈游戏输家](./src/p2682_find_the_losers_of_the_circular_game.rs) (哈希表+模拟)
- [p2815. max pair sum in an array 数组中的最大数对和](./src/p2815_max_pair_sum_in_an_array.rs)
- [p2869. minimum operations to collect elements 收集元素的最少操作次数](./src/p2869_minimum_operations_to_collect_elements.rs) (哈希集)

## 链表

不要用Rust写链表！

- [p19. remove nth node from end of list 删除链表的倒数第N个结点](./src/p19_remove_nth_node_from_end_of_list.py) (双指针)
- [p82. remove duplicates from sorted list ii 删除排序链表中的重复元素 II](./src/p82_remove_duplicates_from_sorted_list_ii.py)
- [p83. remove duplicates from sorted list 删除排序链表中的重复元素](./src/p83_remove_duplicates_from_sorted_list.py)
- [p92. reverse linked list ii 反转链表II](./src/p92_reverse_linked_list_ii.py)
- [p141. linked list cycle 环形链表](./src/p141_linked_list_cycle.py) (快慢指针)
- [p142. linked list cycle ii 环形链表 II](./src/p142_linked_list_cycle_ii.py) (快慢指针)
- [p143. reorder list 重排链表](./src/p143_reorder_list.py) (快慢指针)
- [p206. reverse linked list 反转链表](./src/p206_reverse_linked_list.rs)
- [p237. delete node in a linked list 删除链表中的节点](./src/p237_delete_node_in_a_linked_list.py)
- [p876. middle of the linked list 链表的中间节点](./src/p876_middle_of_the_linked_list.py)

## 数学

- [p7. reverse integer 整数反转](./src/p7_reverse_integer.rs)
- [p9. palindrome number 回文数](./src/p9_palindrome_number.rs)
- [p48. rotate image 旋转图像](./src/p48_rotate_image.rs) (矩阵)
- [p59. spiral matrix ii 螺旋矩阵 II](./src/p59_spiral_matrix_ii.rs)
- [p73. set matrix zeros 矩阵置零](./src/p73_set_matrix_zeros.rs) (矩阵)
- [p74. search a 2d matrix 搜索二维矩阵](./src/p74_search_a_2d_matrix.rs) (矩阵+二分搜索)
- [p168. excel sheet column title Excel表列名称](./src/p168_excel_sheet_column_title.rs)
- [p189. rotate array 轮转数组](./src/p189_rotate_array.rs) (数学，数组)
- [p202. happy number 快乐数](./src/p202_happy_number.rs)
- [p223. rectangle area 矩形面积](./src/p223_rectangle_area.rs) (几何)
- [p263. ugly number 丑数](./src/p263_ugly_number.rs) (数学，对循环的理解)
- [p326. power of three 3的幂](./src/p326_power_of_three.rs) (数学、理解循环)
- [p461. hamming distance 汉明距离](./src/p461_hanming_distance.rs)
- [p1071. greatest common divisor of strings 字符串的最大公因子](./src/p1071_greatest_common_divisor_of_strings.rs)
- [p1281. subtract the product and sum of digits of an integer 整数的各位积和之差](./src/p1281_subtract_the_product_and_sum_of_digits_of_an_integer.rs)
- [p1431. kids with the greatest number of candies 拥有最多糖果的孩子](./src/p1431_kids_with_the_greatest_number_of_candies.rs)
- [p1486. xor operation in an array 数组异或操作](./src/p1486_xor_operation_in_an_array.rs)
- [p1572. matrix diagonal sum 矩阵对角线元素的和](./src/p1572_matrix_diagonal_sum.rs)
- [p1689. partitioning into minimum number of deci-binary numbers 十-二进制数的最少数目](./src/p1689_partitioning_into_minimum_number_of_deci_binary_numbers.rs)
- [p2178. maximum split of positive even integers 拆分成最多数目的正偶数之和](./src/p2178_maximum_split_of_positive_even_integers.rs) (数学+贪心)
- [p2235. add two integers 两整数相加](./src/p2235_add_two_integers.rs)
- [p2352. equal row and column pairs 相等行列对](./src/p2352_equal_row_and_column_pairs.rs) (矩阵+哈希表)
- [p2500. delete greatest value in each row 删除每行中的最大值](./src/p2500_delete_greatest_value_in_each_row.rs) (矩阵+排序)
- [p2520. count the digits that divide a number 统计能整除数字的位数](./src/p2520_count_the_digits_that_divide_a_number.rs) (取余)
- [p2544. alternating digit num 交替数字和](./src/p2544_alternating_digit_sum.rs)
- [p2545. sort the students by their kth score 根据第 K 场考试的分数排序](./src/p2545_sort_the_students_by_their_kth_score.rs) (矩阵+排序)
- [p2582. pass the pillow 递枕头](./src/p2582_pass_the_pillow.rs)
- [p2652. sum multiples 倍数求和](./src/p2652_sum_multiples.rs)

## 递归

递归问题可以分解成原问题和子问题，原问题和子问题是相似的。从原问题到子问题的过程适合用递归来解决。

子问题的规模比原问题小，从原问题分解为子问题称为**递**。不断递下去，总会由尽头，即到达递归的边界条件，此时直接返回它的答案，这称为**归**。

写递归代码重要的是，把边界条件和非边界条件的逻辑写对。

## 树

- [p104. maximum depth of binary tree 二叉树的最大深度](./src/p104_maximum_depth_of_binary_tree.py) (二叉树+递归)
- [p1448. count good nodes in binary tree 统计二叉树中好节点的数目](./src/p1448_count_good_nodes_in_binary_tree.py) (二叉树+递归)
- [p2236. root equals sum of children 判断根结点是否等于子结点之和](./src/p2236_root_equals_sum_of_children.py) (二叉树)

## 分治

## 动态规划

动态规划要点：递推公式

常见动态规划类型：基础、背包问题、打家劫舍、股票问题、子序列问题。

- [p53. maximum subarray 最大子数组和](./src/p53_maximum_subarray.rs) (经典题目)
- [p70. climbing stairs 爬楼梯](./src/p70_climbing_stairs.rs)
- [p121. best time to buy and sell stock 买卖股票的最佳时机](./src/p121_best_time_to_buy_and_sell_stock.rs)
- [p509. fibonacci number 斐波那契数](./src/p509_fibonacci_number.rs)
- [p746. min cost climbing stairs 使用最小花费爬楼梯](./src/p746_min_cost_climbing_stairs.rs)

## 贪心

- [p12. integer to roman 整数转罗马数字](./src/p12_integer_to_roman.rs) (贪心+哈希表)
- [p122. best time to buy and sell stock ii 买卖股票的最佳时机 II](./src/p122_best_time_to_buy_and_sell_stock_ii.rs) (贪心)
- [p605. can place flowers 种花问题](./src/p605_can_place_flowers.rs)

## 回溯

- [p17. letter combinations of a phone number 电话号码的字母组合](./src/p17_letter_combinations_of_a_phone_number.rs) (经典题目)

## 其他

- [p1822. sign of the product of an array 数组元素积的符号](./src/p1822_sign_of_the_product_of_an_array.rs) (遍历)
- [p2089. find target indices after sorting array 找出数组排序后的目标下标](./src/p2089_find_target_indices_after_sorting_array.rs) (排序+遍历)
- [p2240. number of ways to buy pens and pencils 买钢笔和铅笔的方案数](./src/p2240_number_of_ways_to_buy_pens_and_pencils.rs) (枚举)
- [p2525. categorize box according to criteria 根据规则将箱子分类](./src/p2525_categorize_box_according_to_criteria.rs) (模拟)
