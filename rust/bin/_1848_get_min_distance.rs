struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let start = start as usize;
        let mut k = 0;
        loop {
            if start >= k && nums[start - k] == target ||
            start + k < nums.len() && nums[start + k] == target {
                return k as _;
            }
            k += 1;
        }
    }
}


fn main() {
    let tests = vec![
        ( vec![ 1, 2, 3, 4, 5 ], 5, 3, 1 ),
        ( vec![ 1 ], 1, 0, 0 ),
        ( vec![ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ], 1, 0, 0 ),
    ];

    for (nums, target, start, ans) in tests {
        assert_eq!(Solution::get_min_distance(nums, target, start), ans);
    }
}
