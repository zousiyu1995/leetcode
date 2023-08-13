#[test]
fn test() {
    use method1::top_k_frequent;

    let nums1: Vec<i32> = [1, 1, 1, 2, 2, 3].to_vec();
    assert_eq!(top_k_frequent(nums1, 2), [1, 2].to_vec());

    let nums2: Vec<i32> = [1].to_vec();
    assert_eq!(top_k_frequent(nums2, 1), [1].to_vec());
}

// 用 hashmap 统计出现频次，然后按 value 排序
mod method1 {
    #[allow(unused)]
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 用 hashmap 统计出现频次
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.into_iter().for_each(|x| {
            *map.entry(x).or_insert(0) += 1;
        });

        // 将 hashmap 转换为 Vec<(key, value)>，按 value 排序
        let mut hash_vec: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
        hash_vec.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        hash_vec.iter().take(k as usize).map(|(k, _)| *k).collect()
    }
}

// 小顶堆：是二叉树，不过父节点元素总是比子节点小
// 堆里面维护 k 个元素，当遍历完整个数组，堆中的元素就是前 k 个高频元素
// 暂时还不会
mod method2 {
    #[allow(unused)]
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::{BinaryHeap, HashMap};

        // 用 hashmap 统计出现频次
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.into_iter().for_each(|x| {
            *map.entry(x).or_insert(0) += 1;
        });

        // 维护一个大小为 k 的小顶堆
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();

        vec![]
    }
}
