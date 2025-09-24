struct Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut vals: Vec<(usize, i32)> = Vec::new(); // 辅助数组
        for i in 0..n {
            vals.push((i, nums[i]));
        }
        // 按照数值降序排序
        vals.sort_by(|x1, x2| x2.1.cmp(&x1.1));
        // 取前 k 个并按照下标升序排序
        vals[0..k as usize].sort_by(|x1, x2| x1.0.cmp(&x2.0));
        let mut res: Vec<i32> = Vec::new(); // 目标子序列
        for i in 0..k as usize {
            res.push(vals[i].1);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![2, 1, 3, 3], 2, vec![3, 3]),
        (vec![-1, -2, 3, 4], 3, vec![-1, 3, 4]),
        (vec![3, 4, 3, 3], 2, vec![3, 4]),
    ];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::max_subsequence(nums, k), expected,);
    }
}
