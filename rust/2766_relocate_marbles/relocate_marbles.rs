struct Solution;

impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut mp = HashMap::new();
        let mut ans = Vec::new();
        for &num in &nums {
            mp.insert(num, true);
        }
        for i in 0..move_from.len() {
            mp.remove(&move_from[i]);
            mp.insert(move_to[i], true);
        }
        for (&key, _) in &mp {
            ans.push(key);
        }
        ans.sort();
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 6, 7, 8],
            vec![1, 7, 2],
            vec![2, 9, 5],
            vec![5, 6, 8, 9],
        ),
        (vec![1, 1, 3, 3], vec![1, 3], vec![2, 2], vec![2]),
    ];

    for (nums, move_from, move_to, ans) in tests {
        assert_eq!(Solution::relocate_marbles(nums, move_from, move_to), ans);
    }
}
