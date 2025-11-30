struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut cnt = std::collections::HashMap::new();
        let mut ans = vec![0; nums.len() - k as usize + 1];
        for i in 0..nums.len() {
            *cnt.entry(nums[i]).or_insert(0) += 1;
            if i < k as usize - 1 {
                continue;
            }
            if i >= k as usize {
                if cnt.get(&nums[i - k as usize]).unwrap() == &1 {
                    cnt.remove(&nums[i - k as usize]);
                } else {
                    *cnt.entry(nums[i - k as usize]).or_default() -= 1;
                }
            }
            if cnt.len() <= x as usize {
                for e in &cnt {
                    ans[i - k as usize + 1] += e.0 * e.1;
                }
                continue;
            }
            let mut num_cnts = cnt.iter().collect::<Vec<(&i32, &i32)>>();
            num_cnts.sort_by(|a, b| {
                if a.1 == b.1 {
                    b.0.cmp(&a.0)
                } else {
                    b.1.cmp(&a.1)
                }
            });
            for j in 0..x as usize {
                ans[i - k as usize + 1] += num_cnts[j].0 * num_cnts[j].1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2, vec![6, 10, 12]),
        (vec![3, 8, 7, 8, 7, 5], 2, 2, vec![11, 15, 15, 15, 12]),
    ];

    for (nums, k, x, expected) in tests {
        assert_eq!(Solution::find_x_sum(nums, k, x), expected);
    }
}
