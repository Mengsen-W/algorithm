struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut count: HashMap<i32, i32> = HashMap::new();
        for a in nums {
            *count.entry(a).or_insert(0) += 1;
        }
        let mut maxf = 0;
        for &freq in count.values() {
            if freq > maxf {
                maxf = freq;
            }
        }
        let mut res = 0;
        for &freq in count.values() {
            if freq == maxf {
                res += maxf;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 2, 3, 1, 4], 4), (vec![1, 2, 3, 4, 5], 5)];

    for (nums, expected) in tests {
        assert_eq!(Solution::max_frequency_elements(nums), expected);
    }
}
