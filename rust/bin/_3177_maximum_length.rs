struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut dp = HashMap::new();
        let mut zd = vec![0; k as usize + 1];

        for &v in &nums {
            let tmp = dp.entry(v).or_insert(vec![0; k as usize + 1]);
            for j in 0..=k as usize {
                tmp[j] += 1;
                if j > 0 {
                    tmp[j] = tmp[j].max(zd[j - 1] + 1);
                }
            }

            for j in 0..=k as usize {
                zd[j] = zd[j].max(tmp[j]);
            }
        }
        zd[k as usize]
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 1, 1, 3], 2, 4), (vec![1, 2, 3, 4, 5, 1], 0, 2)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::maximum_length(nums, k), ans);
    }
}
