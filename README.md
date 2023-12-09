# My leetcode solutions

Written in `rust` and `python`.

- [My leetcode solutions](#my-leetcode-solutions)
  - [二分查找](#二分查找)
  - [双指针](#双指针)
    - [滑移窗口](#滑移窗口)
  - [前缀和、后缀和、差分](#前缀和后缀和差分)
  - [字符串](#字符串)
  - [栈和队列](#栈和队列)
    - [单调栈](#单调栈)
  - [堆](#堆)
  - [哈希](#哈希)
  - [链表](#链表)
  - [递归](#递归)
  - [树](#树)
  - [分治](#分治)
  - [动态规划](#动态规划)
  - [贪心](#贪心)
  - [回溯](#回溯)
  - [数学](#数学)
    - [矩阵](#矩阵)
    - [位运算](#位运算)
  - [其他](#其他)

## 二分查找

- [p33. 搜索旋转排序数组](./src/p33.rs) (经典题目)
- [p34. 在排序数组中查找元素的第一个和最后一个位置](./src/p34.rs) (经典题目)
- [p35. 搜索插入位置](./src/p35_search_insert.rs)
- [p69. x的平方根](./src/p69_my_sqrt.rs)
- [p81. 搜索旋转排序数组 II](./src/p81.rs) (经典题目)
- [p153. 寻找旋转排序数组中的最小值](./src/p153_find_minimum_in_rotated_sorted_array.rs)
- [p162. 寻找峰值](./src/p162_find_peak_element.rs)
- [p274. H指数](./src/p274_h_index.rs)
- [p275. H指数 II](./src/p275_h_index_ii.rs)
- [p278. f第一个错误的版本](./src/p278_first_bad_version.rs)
- [p367. 有效的完全平方数](./src/p367_valid_perfect_square.rs)
- [p374. 猜数字大小](./src/p374_guess_number_higher_or_lower.rs)
- [p540. 有序数组中的单一元素](./src/p540_single_element_in_a_sorted_array.rs)
- [p704. 二分查找](./src/p704_binary_search.rs)
- [p852. 山脉数组的峰顶索引](./src/p852_peak_index_in_a_mountain_array.rs)
- [p2300. 咒语和药水的成功对数](./src/p2300_successful_pairs_of_spells_and_potions.rs)

## 双指针

- [p4. 寻找两个正序数组的中位数](./src/p4_median_of_two_sorted_arrays.rs)
- [p11. 盛最多水的容器](./src/p11_container_with_most_water.rs)
- [p15. 三数之和](./src/p15_3sum.rs)
- [p26. 删除有序数组中的重复项](./src/p26_remove_duplicates_from_sorted_array.rs)
- [p27. 移除元素](./src/p27_reomove_element.rs)
- [p56. 合并区间](./src/p56_merge_intervals.rs) (排序，区间问题，经典题目)
- [p57. 插入区间](./src/p57_insert_interval.rs) (排序，区间问题，经典题目)
- [p80. 删除有序数组中的重复项 II](./src/p80_remove_duplicates_from_sorted_array_ii.rs)
- [p88. 合并两个有序数组](./src/p88_merge_sorted_array.rs)
- [p167. 两数之和 II 输入有序数组](./src/p167_two_sum_ii_input_array_is_sorted.rs) (经典题目)
- [p209. 长度最小的子数组](./src/p209_minimum_size_subarray_sum.rs) (滑移窗口)
- [p228. 汇总区间](./src/p228_summary_ranges.rs) (滑移窗口)
- [p283. 移动零](./src/p283_move_zeroes.rs)
- [p443. 压缩字符串](./src/p443_string_compression.rs) (双指针+字符串)
- [p633. 平方数之和](./src/p633_sum_of_square_numbers.rs) (双指针，和p167类似)
- [p643. 子数组最大平均数 I](./src/p643_maximum_average_subarray_i.rs) (滑移窗口)
- [p680. 验证回文串 II](./src/p680_valid_palindrome_ii.rs) (双指针，经典)
- [p713. 乘积小于K的子数组](./src/p713_subarray_product_less_than_k.rs)
- [p849. 到最近的人的最大距离](./src/p849_maximize_distance_to_closest_person.rs)
- [p977. 有序数组的平方](./src/p977_squares_of_a_sorted_array.rs)
- [p986. 区间列表的交集](./src/p986_interval_list_intersections.rs) (双指针，经典题目)
- [p2461. 长度为K子数组中的最大和](./src/p2461.py)
- [p2562. 找出数组的串联值](./src/p2562_find_the_array_concatenation_value.rs) (双指针+模拟)
- [p2824. 统计和小于目标的下标对数目](./src/p2824_count_pairs_whose_sum_is_less_than_target.py) (排序+双指针)

### 滑移窗口

- [p30. 串联所有单词的子串](./src/p30.py) (经典题目)
- [p76. 最小覆盖子串](./src/p76.py) (经典题目)
- [p187. 重复的DNA序列](./src/p187_repeated_dna_sequences.rs)
- [p217. 存在重复元素](./src/p217.py)
- [p219. 存在重复元素 II](./src/p219.py)
- [p424. 替换后的最长重复字符](./src/p424.py)
- [p438. 找到字符串中所有字母异位词](./src/p438_find_all_anagrams_in_a_string.rs)
- [p567. 字符串的排列](./src/p567.py)
- [p594. 最长和谐子序列](./src/p594.py)
- [p904. 水果成篮](./src/p904.py)
- [p1004. 最大连续1的个数III](./src/p1004.py)
- [p1052. 爱生气的书店老板](./src/p1052.py)
- [p1343. 大小为K且平均值大于等于阈值的子数组数目](./src/p1343.py)
- [p1423. 可获得的最大点数](./src/p1423.rs)
- [p1438. 绝对差不超过限制的最长连续子数组](./src/p1438.py)
- [p1456. 定长子串中元音的最大数目](./src/p1456.rs)
- [p1838. 最高频元素的频数](./src/p1838.py)
- [p1984. 学生分数的最小差值](./src/p1984.py)
- [p2090. 半径为k的子数组平均值](./src/p2090.py)
- [p2269. 找到一个数字的K美丽值](./src/p2269_find_the_k_beauty_of_a_number.rs)
- [p2379. 得到K个黑块的最少涂色次数](./src/p2379.py)
- [p2653. 滑动子数组的美丽值](./src/p2653.py)
- [p2841. 几乎唯一子数组的最大和](./src/p2841.py)

## 前缀和、后缀和、差分

- [p42. 接雨水](./src/p42_trapping_rain_water.rs) (前缀最大值、后缀最大值)
- [p238. 除自身以外数组的乘积](./src/p238_product_of_array_except_self.rs)
- [p303. 区域和检索——数组不可变](./src/p303_range_sum_query_immutable.rs)
- [p523. 连续的子数组和](./src/p523_continuous_subarray_sum.rs) (前缀和+哈希表，经典题)
- [p525. 连续数组](./src/p525_contiguous_array.rs) (前缀和+哈希表，经典题)
- [p560. 和为K的子数组](./src/p560_subarray_sum_equals_k.rs) (前缀和+哈希表，经典题)
- [p724. 寻找数组的中心下标](./src/p724_find_pivot_index.rs)
- [p1480. 一维数组的动态和](./src/p1480_running_sum_of_1d_array.rs) (前缀和)
- [p1685. 有序数组中差绝对值之和](./src/p1685_sum_of_absolute_differences_in_a_sorted_array.rs) (前缀和+后缀和)
- [p1732. 找到最高海拔](./src/p1732_find_the_highest_altitude.rs)
- [p1749. 任意子数组和的绝对值的最大值](./src/p1749_maximum_absolute_sum_of_any_subarray.rs)

## 字符串

- [p5. 最长回文子串](./src/p5_longest_palindromic_substring.rs) (中心扩散法)
- [p6. N字形变换](./src/p6_zigzag_conversion.rs) (模拟 || 观察规律)
- [p8. 字符串转换整数（atoi）](./src/p8_string_to_integer_atoi.rs)
- [p14. 最长公共前缀](./src/p14_longest_common_prefix.rs)
- [p26. 找出字符串中第一个匹配项的下标](./src/p28_find_the_index_of_the_first_occurrence_in_a_string.rs)
- [p28. 找出字符串中第一个匹配项的下标](./src/p28_find_the_index_of_the_first_occurrence_in_a_string.rs)
- [p58. 最后一个单词的长度](./src/p58_length_of_last_word.py)
- [p125. 验证回文串](./src/p125_valid_palindrome.rs) (双指针)
- [p151. 反转字符串中的单词](./src/p151_reverse_words_in_a_string.rs)
- [p344. 反转字符串](./src/p344_reverse_string.rs)
- [p345. 反转字符串中的元音字母](./src/p345_reverse_vowels_of_a_string.rs)
- [p347. 前k个高频元素](./src/p347_top_k_frequent_elements.rs)
- [p459. 重复的子字符串](./src/p459_repeated_substring_pattern.rs)
- [p541. 反转字符串 II](./src/p541_reverse_string_ii.rs)
- [p1507. 转变日期格式](./src/p1507_reformat_data.rs)
- [p1657. 确定两个字符串是否接近](./src/p1657_determine_if_two_strings_are_close.rs) (哈希表)
- [p1768. 交替合并字符串](./src/p1768_merge_strings_alternately.rs)
- [p1869. 哪种连续子字符串更长](./src/p1869_longer_contiguous_segments_of_ones_than_zeros.rs)
- [p1876. 长度为三且各字符不同的子字符串](./src/p1876_substrings_of_size_three_with_distinct_characters.rs)
- [p2011. 执行操作后的变量值](./src/p2011_final_value_of_variable_after_performing_operations.rs) (模拟)
- [p2337. 移动片段得到字符串](./src/p2337_move_pieces_to_obtain_a_string.rs)
- [p2586. 统计范围内的元音字符串数](./src/p2586_count_the_number_of_vowel_strings_in_range.rs) (哈希表+简单模拟)
- [p2678. 老人的数目](./src/p2678_number_of_senior_citizens.rs) (字符串parse)
- [p2828. 判别首字母缩略词](./src/p2828_check_if_a_string_is_an_acronym_of_words.rs)

## 栈和队列

- [p20. 有效的括号](./src/p20_valid_parentheses.rs)
- [p32. 最长有效括号](./src/p32_longest_valid_parentheses.rs)
- [p71. 简化路径](./src/p71_simplify_path.rs) (栈+字符串)
- [p150. 逆波兰表达式求值](./src/p150_evaluate_reverse_polish_notation.rs)
- [p155. 最小栈](./src/p155_min_stack.rs)
- [p225. 用队列实现栈](./src/p225_implement_stack_using_queues.rs)
- [p232. 用栈实现队列](./src/p232_implement_queue_using_stacks.rs)
- [p239. 滑动窗口最大值](./src/p239_sliding_window_maximum.rs)
- [p394. 字符串解码](./src/p394_decode_string.rs) (经典题目)
- [p1047. 删除字符串中的所有相邻重复项](./src/p1047_remove_all_adjacent_duplicates_in_string.rs)
- [p2216. 美化数组的最少删除数](./src/p2216_minimum_deletions_to_make_array_beautiful.rs)
- [p2810. 故障键盘](./src/p2810_faulty_keyboard.rs)
- [p2390. 从字符串中移除星号](./src/p2390_removing_stars_from_a_string.rs)

### 单调栈

- [p84. 柱状图中最大的矩形](./src/p84_largest_rectangle_in_histogram.rs) (单调栈)
- [p496. 下一个更大元素 I](./src/p496_next_greater_element_i.rs) (单调栈+哈希表)
- [p503. 下一个更大元素 II](./src/p503_next_greater_element_ii.rs) (单调栈)
- [p739. 每日温度](./src/p739_daily_temperatures.rs) (单调栈)

## 堆

- [p215. 数组中的第K个最大元素](./src/p215_kth_largest_element_in_an_array.rs) (大顶堆)
- [p264. 丑数 II](./src/p264_ugly_number_ii.rs) (小顶堆)
- [p1962. 移除石子使总数最小](./src/p1962_remove_stones_to_minimize_the_total.rs) (大顶堆)
- [p2208. 将数组和减半的最少操作次数](./src/p2208_minimum_operations_to_halve_array_sum.rs) (大顶堆)
- [p2233. K次增加后的最大成绩](./src/p2233_maximum_product_after_k_increments.rs) (小顶堆)
- [p2530. 执行K次操作后的最大分数](./src/p2530_maximal_score_after_applying_k_operations.rs) (贪心+大顶堆)
- [p2558. 从数量最多的堆取走礼物](./src/p2558_take_gifts_from_the_richest_pile.rs) (大顶堆)

## 哈希

- [p1. 两数之和](./src/p1_two_sum.rs)
- [p3. 无重复字符的最长子串](./src/p3_longest_substring_without_repeating_characters.rs)
- [p13. 罗马数字转整数](./src/p13_roman_to_integer.rs)
- [p36. 有效的数独](./src/p36_valid_sudoku.rs)
- [p41. 缺失的第一个正数](./src/p41_first_missing_positive.rs) (哈希表)
- [p49. 字母异位词分组](./src/p49_group_anagrams.rs)
- [p128. 最长连续序列](./src/p128_longest_consecutive_sequence.rs) (经典题目)
- [p136. 只出现一次的数字](./src/p136_single_number.rs) (哈希表、异或)
- [p137. 只出现一次的数字 II](./src/p137_single_number_ii.rs) (哈希表、位运算)
- [p229. 多数元素 II](./src/p229_majority_element_ii.rs) (哈希表)
- [p242. 有效的字母异位词](./src/p242_valid_anagram.rs)
- [p260. 只出现一次的数字 III](./src/p260_single_number_iii.rs) (哈希表、位运算)
- [p268. 丢失的数字](./src/p268_missing_number.rs) (数学、哈希集)
- [p290. 单词规律](./src/p290_word_pattern.rs)
- [p349. 两个数组的交集](./src/p349_intersection_of_two_arrays.rs)
- [p448. 找到所有数组中消失的数字](./src/p448_find_all_numbers_disappeared_in_an_array.rs) (原地哈希)
- [p454. 四数相加 II](./src/p454_4sum_ii.rs)
- [p1207. 独一无二的出现次数](./src/p1207_unique_number_of_occurrences.rs)
- [p1267. 统计参与通信的服务器](./src/p1267_count_servers_that_communicate.rs)
- [p1410. HTML实体解析器](./src/p1410_html_entity_parser.rs)
- [p1726. 同积元组](./src/p1726_tuple_with_same_product.rs) (哈希表+组合数学)
- [p2103. 环和杆](./src/p2103_rings_and_rods.rs) (哈希表+字符串)
- [p2215. 找出两数组的不同](./src/p2215_find_the_difference_of_two_arrays.rs)
- [p2273. 移除字母异位词后的结果数组](./src/p2273_find_resultant_array_after_removing_anagrams.rs)
- [p2341. 数组能形成多少数对](./src/p2341_maximum_number_of_pairs_in_array.rs)
- [p2342. 数位和相等数对的最大和](./src/p2342_max_sum_of_a_pair_with_equal_sum_of_digits.rs) (哈希表+最大堆)
- [p2367. 算术三元组的数目](./src/p2367_number_of_arithmetic_triplets.rs)
- [p2395. 和相等的子数组](./src/p2395_find_subarrays_with_equal_sum.rs)
- [p2605. 从两个数字数组里生成最小数字](./src/p2605_form_smallest_number_from_two_digit_arrays.rs)
- [p2682. 找出转圈游戏输家](./src/p2682_find_the_losers_of_the_circular_game.rs) (哈希表+模拟)
- [p2815. 数组中的最大数对和](./src/p2815_max_pair_sum_in_an_array.rs)
- [p2869. 收集元素的最少操作次数](./src/p2869_minimum_operations_to_collect_elements.rs) (哈希集)

## 链表

不要用Rust写链表！

- [p19. 删除链表的倒数第N个结点](./src/p19_remove_nth_node_from_end_of_list.py) (双指针)
- [p21. 合并两个有序链表](./src/p21_merge_two_sorted_lists.py)
- [p82. 删除排序链表中的重复元素 II](./src/p82_remove_duplicates_from_sorted_list_ii.py)
- [p83. 删除排序链表中的重复元素](./src/p83_remove_duplicates_from_sorted_list.py)
- [p92. 反转链表II](./src/p92_reverse_linked_list_ii.py)
- [p141. 环形链表](./src/p141_linked_list_cycle.py) (快慢指针)
- [p142. 环形链表 II](./src/p142_linked_list_cycle_ii.py) (快慢指针)
- [p143. 重排链表](./src/p143_reorder_list.py) (快慢指针)
- [p160. 相交链表](./src/p160_intersection_of_two_linked_lists.py) (哈希表、双指针)
- [p206. 反转链表](./src/p206_reverse_linked_list.rs)
- [p237. 删除链表中的节点](./src/p237_delete_node_in_a_linked_list.py)
- [p234. 回文链表](./src/p234_palindrome_linked_list.py) (双指针)
- [p876. 链表的中间节点](./src/p876_middle_of_the_linked_list.py)

## 递归

## 树

- [p104. 二叉树的最大深度](./src/p104_maximum_depth_of_binary_tree.py) (二叉树+递归)
- [p1448. 统计二叉树中好节点的数目](./src/p1448_count_good_nodes_in_binary_tree.py) (二叉树+递归)
- [p2236. 判断根结点是否等于子结点之和](./src/p2236_root_equals_sum_of_children.py) (二叉树)

## 分治

- [p50. Pow(x, n)](./src/p50_powx_n.rs)

## 动态规划

- [p53. 最大子数组和](./src/p53_maximum_subarray.rs) (经典题目)
- [p70. 爬楼梯](./src/p70_climbing_stairs.rs)
- [p121. 买卖股票的最佳时机](./src/p121_best_time_to_buy_and_sell_stock.rs)
- [p509. 斐波那契数](./src/p509_fibonacci_number.rs)
- [p746. 使用最小花费爬楼梯](./src/p746_min_cost_climbing_stairs.rs)

## 贪心

- [p12. 整数转罗马数字](./src/p12_integer_to_roman.rs) (贪心+哈希表)
- [p122. 买卖股票的最佳时机 II](./src/p122_best_time_to_buy_and_sell_stock_ii.rs)
- [p135. 分发糖果](./src/p135_candy.rs)
- [p406. 根据身高重建队列](./src/p406_queue_reconstruction_by_height.rs) (排序+贪心，经典题目)
- [p435. 无重叠区间](./src/p435_non_overlapping_intervals.rs) (经典题目)
- [p452. 用最少数量的箭引爆气球](./src/p452_minimum_number_of_arrows_to_burst_balloons.rs) (经典题目)
- [p455. 分发饼干](./src/p455_assign_cookies.rs)
- [p605. 种花问题](./src/p605_can_place_flowers.rs)
- [p665. 非递减数列](./src/p665_non_decreasing_array.rs) (经典题目)
- [p763. 划分字母区间](./src/p763_partition_labels.rs) (贪心+哈希表)

## 回溯

- [p17. 电话号码的字母组合](./src/p17_letter_combinations_of_a_phone_number.rs) (经典题目)

## 数学

- [p7. 整数反转](./src/p7_reverse_integer.rs)
- [p9. 回文数](./src/p9_palindrome_number.rs)
- [p168. Excel表列名称](./src/p168_excel_sheet_column_title.rs)
- [p189. 轮转数组](./src/p189_rotate_array.rs) (数学，数组)
- [p202. 快乐数](./src/p202_happy_number.rs)
- [p223. 矩形面积](./src/p223_rectangle_area.rs) (几何)
- [p263. 丑数](./src/p263_ugly_number.rs) (数学，对循环的理解)
- [p326. 3的幂](./src/p326_power_of_three.rs) (数学、理解循环)
- [p461. 汉明距离](./src/p461_hanming_distance.rs)
- [p1071. 字符串的最大公因子](./src/p1071_greatest_common_divisor_of_strings.rs)
- [p1281. 整数的各位积和之差](./src/p1281_subtract_the_product_and_sum_of_digits_of_an_integer.rs)
- [p1431. 拥有最多糖果的孩子](./src/p1431_kids_with_the_greatest_number_of_candies.rs)
- [p1689. 十-二进制数的最少数目](./src/p1689_partitioning_into_minimum_number_of_deci_binary_numbers.rs)
- [p2178. 拆分成最多数目的正偶数之和](./src/p2178_maximum_split_of_positive_even_integers.rs) (数学+贪心)
- [p2235. 两整数相加](./src/p2235_add_two_integers.rs)
- [p2520. 统计能整除数字的位数](./src/p2520_count_the_digits_that_divide_a_number.rs) (取余)
- [p2544. 交替数字和](./src/p2544_alternating_digit_sum.rs)
- [p2582. 递枕头](./src/p2582_pass_the_pillow.rs)
- [p2652. 倍数求和](./src/p2652_sum_multiples.rs)
- [p2656. K个元素的最大和](./src/p2656_maximum_sum_with_exactly_k_elements.rs)

### 矩阵

- [p48. 旋转图像](./src/p48_rotate_image.rs)
- [p59. 螺旋矩阵 II](./src/p59_spiral_matrix_ii.rs)
- [p73. 矩阵置零](./src/p73_set_matrix_zeros.rs)
- [p74. 搜索二维矩阵](./src/p74_search_a_2d_matrix.rs) (矩阵+二分搜索)
- [p240. 搜索二维矩阵 II](./src/p240_search_a_2d_matrix_ii.rs) (矩阵+二分搜索)
- [p1572. 矩阵对角线元素的和](./src/p1572_matrix_diagonal_sum.rs)
- [p2352. 相等行列对](./src/p2352_equal_row_and_column_pairs.rs) (矩阵+哈希表)
- [p2500. 删除每行中的最大值](./src/p2500_delete_greatest_value_in_each_row.rs) (矩阵+排序)
- [p2545. 根据第K场考试的分数排序](./src/p2545_sort_the_students_by_their_kth_score.rs) (矩阵+排序)

### 位运算

- [p318. 最大单词长度乘积](./src/p318_maximum_product_of_word_lengths.rs)
- [p1486. 数组异或操作](./src/p1486_xor_operation_in_an_array.rs)

## 其他

- [p485. 最大连续1的个数](./src/p485.py)
- [p1822. 数组元素积的符号](./src/p1822_sign_of_the_product_of_an_array.rs) (遍历)
- [p2089. 找出数组排序后的目标下标](./src/p2089_find_target_indices_after_sorting_array.rs) (排序+遍历)
- [p2240. 买钢笔和铅笔的方案数](./src/p2240_number_of_ways_to_buy_pens_and_pencils.rs) (枚举)
- [p2303. 计算应缴税款总额](./src/p2303_calculate_amount_paid_in_taxes.rs) (模拟)
- [p2525. 根据规则将箱子分类](./src/p2525_categorize_box_according_to_criteria.rs) (模拟)
- [p2609. 最长平衡子字符串](./src/p2609_find_the_longest_balanced_substring_of_a_binary_string.rs) (模拟)
