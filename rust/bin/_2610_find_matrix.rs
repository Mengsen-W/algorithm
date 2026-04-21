struct Solution;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut cnt = HashMap::new();
        for &x in &nums {
            *cnt.entry(x).or_insert(0) += 1;
        }

        let mut ans = Vec::new();
        while !cnt.is_empty() {
            let mut arr = Vec::new();
            let keys: Vec<i32> = cnt.keys().cloned().collect();
            for key in keys {
                if let Some(value) = cnt.get_mut(&key) {
                    *value -= 1;
                    arr.push(key);
                    if *value == 0 {
                        cnt.remove(&key);
                    }
                }
            }
            ans.push(arr);
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 3, 4, 1, 2, 3, 1],
            vec![vec![2, 4, 1, 3], vec![1, 3], vec![1]],
        ),
        (vec![1, 2, 3, 4], vec![vec![4, 3, 2, 1]]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::find_matrix(nums), ans);
    }
}
