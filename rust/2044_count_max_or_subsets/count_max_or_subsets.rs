struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let (mut max_or, mut cnt) = (0, 0);

        fn dfs(pos: usize, or_val: i32, nums: &Vec<i32>, max_or: &mut i32, cnt: &mut i32) {
            if pos == nums.len() {
                if or_val > *max_or {
                    *max_or = or_val;
                    *cnt = 1;
                } else if or_val == *max_or {
                    *cnt += 1;
                }
                return;
            }
            dfs(pos + 1, or_val | nums[pos], nums, max_or, cnt);
            dfs(pos + 1, or_val, nums, max_or, cnt);
        }

        dfs(0, 0, &nums, &mut max_or, &mut cnt);
        cnt
    }
}

fn main() {
    let tests = vec![(vec![3, 1], 2), (vec![2, 2, 2], 7), (vec![3, 2, 1, 5], 6)];

    for (nums, expected) in tests {
        assert_eq!(Solution::count_max_or_subsets(nums), expected);
    }
}
