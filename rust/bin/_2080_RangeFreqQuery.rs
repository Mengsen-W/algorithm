struct Solution;

use std::collections::HashMap;

struct RangeFreqQuery {
    // 数值为键，出现下标数组为值的哈希表
    occurrence: HashMap<i32, Vec<usize>>,
}

impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut occurrence = HashMap::new();
        // 顺序遍历数组初始化哈希表
        for (i, &num) in arr.iter().enumerate() {
            occurrence.entry(num).or_insert_with(Vec::new).push(i);
        }
        RangeFreqQuery { occurrence }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        // 查找对应的出现下标数组，不存在则为空
        if let Some(pos) = self.occurrence.get(&value) {
            // 两次二分查找计算子数组内出现次数
            let l = self.lower_bound(pos, left as usize);
            let r = self.upper_bound(pos, right as usize);
            return (r - l) as i32;
        }
        0
    }

    // 查找下界
    fn lower_bound(&self, pos: &Vec<usize>, target: usize) -> usize {
        let mut low = 0 as i32;
        let mut high = pos.len() as i32 - 1;
        while low <= high {
            let mid = low + (high - low) as i32 / 2;
            if pos[mid as usize] < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        low as usize
    }

    // 查找上界
    fn upper_bound(&self, pos: &Vec<usize>, target: usize) -> usize {
        let mut low = 0;
        let mut high = pos.len() as i32 - 1;
        while low <= high {
            let mid = low + (high - low) as i32 / 2;
            if pos[mid as usize] <= target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        low as usize
    }
}

fn main() {
    let range_freq_query = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
    assert_eq!(range_freq_query.query(1, 2, 4), 1);
    assert_eq!(range_freq_query.query(0, 11, 33), 2);
}
