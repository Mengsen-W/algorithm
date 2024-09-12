struct Solution;

impl Solution {
    pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut l = 0;
        let mut r = n / 2;
        let check = |m: usize| -> bool {
            for i in 0..m {
                if nums[i] * 2 > nums[n - m + i] {
                    return false;
                }
            }
            true
        };

        while l < r {
            let m = (l + r + 1) / 2;
            if check(m) {
                l = m;
            } else {
                r = m - 1;
            }
        }
        (l * 2) as i32
    }
}

fn main() {
    let tests = vec![
        (vec![3, 5, 2, 4], 2),
        (vec![9, 2, 5, 4], 4),
        (vec![7, 6, 8], 0),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::max_num_of_marked_indices(nums), ans);
    }
}
