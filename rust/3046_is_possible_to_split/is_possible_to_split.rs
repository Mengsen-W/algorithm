struct Solution;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut count = HashMap::new();
        for num in nums {
            let counter = count.entry(num).or_insert(0);
            *counter += 1;
            if *counter > 2 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let tests = vec![(vec![1, 1, 2, 2, 3, 4], true), (vec![1, 1, 1, 1], false)];

    for (nums, ans) in tests {
        assert_eq!(Solution::is_possible_to_split(nums), ans);
    }
}
