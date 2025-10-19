struct Solution;

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut mp = vec![0; value as usize];
        nums.iter()
            .for_each(|&x| mp[((x % value + value) % value) as usize] += 1);
        let mut mex = 0;
        while mp[(mex % value) as usize] > 0 {
            mp[(mex % value) as usize] -= 1;
            mex += 1;
        }
        mex
    }
}

fn main() {
    let tests = vec![
        (vec![1, -10, 7, 13, 6, 8], 5, 4),
        (vec![1, -10, 7, 13, 6, 8], 7, 2),
    ];

    for (test, value, ans) in tests {
        assert_eq!(Solution::find_smallest_integer(test, value), ans);
    }
}
