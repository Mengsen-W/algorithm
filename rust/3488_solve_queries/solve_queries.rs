struct Solution;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let n = nums.len() as i32;
        let nu = n as usize;
        let mut left = vec![0i32; nu];
        let mut right = vec![0i32; nu];
        let mut pos: HashMap<i32, i32> = HashMap::new();
        for i in -n..n {
            if i >= 0 {
                let idx = i as usize;
                left[idx] = *pos.get(&nums[idx]).unwrap_or(&(i - n));
            }
            let wrap = ((i % n + n) % n) as usize;
            pos.insert(nums[wrap], i);
        }

        pos.clear();
        for i in (0..2 * n).rev() {
            if i < n {
                let idx = i as usize;
                right[idx] = *pos.get(&nums[idx]).unwrap_or(&(i + n));
            }
            let wrap = (i % n) as usize;
            pos.insert(nums[wrap], i);
        }

        queries
            .iter()
            .map(|&q| {
                let x = q as usize;
                let xi = q;
                if xi - left[x] == n {
                    -1
                } else {
                    (xi - left[x]).min(right[x] - xi)
                }
            })
            .collect()
    }
}

fn main() {
    let tests = vec![
        (vec![1,3,1,4,1,3,2], vec![0,3,5], vec![2,-1,3]),
        (vec![1,2,3,4], vec![0,1,2,3], vec![-1,-1,-1,-1]),
    ];

    for (nums, queries, ans) in tests {
        assert_eq!(Solution::solve_queries(nums, queries), ans);
    }
}
