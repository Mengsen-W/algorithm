struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let median = ((n * (n + 1) / 2 + 1) / 2) as i64;
        let mut res = 0;
        let mut lo = 1;
        let mut hi = n as i32;

        // 检测数组中不同元素数目小于等于 t 的连续子数组数目是否大于等于 median
        fn check(nums: &[i32], t: usize, median: i64) -> bool {
            let mut cnt: HashMap<i32, i32> = HashMap::new();
            let mut j = 0;
            let mut tot = 0 as i64;
            for (i, &v) in nums.iter().enumerate() {
                *cnt.entry(v).or_insert(0) += 1;
                while cnt.len() > t {
                    *cnt.entry(nums[j]).or_insert(0) -= 1;
                    if *cnt.get(&nums[j]).unwrap_or(&0) == 0 {
                        cnt.remove(&nums[j]);
                    }
                    j += 1;
                }
                tot += (i - j + 1) as i64;
            }
            tot >= median
        }

        while lo <= hi {
            let mid = (lo + hi) / 2;
            if check(&nums, mid as usize, median) {
                res = mid;
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }

        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], 1),
        (vec![3, 4, 3, 4, 5], 2),
        (vec![4, 3, 5, 4], 2),
    ];

    for (nums, ans) in tests {
      assert_eq!(Solution::med)
    }
}
