struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(
            nums: &Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
            perm: &mut Vec<i32>,
            vis: &mut Vec<bool>,
            idx: usize,
        ) {
            if idx == nums.len() {
                ans.push(perm.clone());
                return;
            }
            for i in 0..nums.len() {
                if vis[i] || (i > 0 && nums[i] == nums[i - 1] && !vis[i - 1]) {
                    continue;
                }
                perm.push(nums[i]);
                vis[i] = true;
                backtrack(nums, ans, perm, vis, idx + 1);
                vis[i] = false;
                perm.pop();
            }
        }
        let mut nums = nums.clone();
        nums.sort();
        let mut ans = Vec::new();
        let mut perm = Vec::new();
        let mut vis = vec![false; nums.len()];
        backtrack(&nums, &mut ans, &mut perm, &mut vis, 0);
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 1, 2],
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
        ),
        (
            vec![1, 2, 3],
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
        ),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::permute_unique(nums), ans);
    }
}
