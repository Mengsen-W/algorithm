struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut bits_max_pos = vec![-1; 31];
        let mut res = i32::MAX;

        for i in 0..n {
            for j in 0..=30 {
                if nums[i] >> j & 1 == 1 {
                    bits_max_pos[j] = i as i32;
                }
            }

            let mut pos_to_bit = Vec::new();
            for j in 0..=30 {
                if bits_max_pos[j] != -1 {
                    pos_to_bit.push((bits_max_pos[j], j as i32));
                }
            }
            pos_to_bit.sort_by(|a, b| b.0.cmp(&a.0));
            let mut val = 0;
            let mut j = 0;
            while j < pos_to_bit.len() {
                let p = j;
                while j < pos_to_bit.len() && pos_to_bit[j].0 == pos_to_bit[p].0 {
                    val |= 1 << pos_to_bit[j].1;
                    j += 1;
                }
                res = std::cmp::min(res, (val - k).abs());
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 4, 5], 3, 0),
        (vec![1, 3, 1, 3], 2, 1),
        (vec![1], 10, 9),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::minimum_difference(nums, k), ans);
    }
}
