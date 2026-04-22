struct Solution;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let reverse_num = |mut x: i32| -> i32 {
            let mut y = 0;
            while x > 0 {
                y = y * 10 + x % 10;
                x /= 10;
            }
            y
        };

        let mut prev = HashMap::new();
        let n = nums.len();
        let mut ans = n + 1;

        for (i, &x) in nums.iter().enumerate() {
            if let Some(&idx) = prev.get(&x) {
                ans = ans.min(i - idx);
            }
            prev.insert(reverse_num(x), i);
        }

        if ans == n + 1 {
            -1
        } else {
            ans as i32
        }
    }
}

fn main() {
    let tests = vec![
        (vec![12,21,45,33,54], 1),
        (vec![120,21], 1),
        (vec![21,120], -1),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::min_mirror_pair_distance(nums), ans);
    }
}

