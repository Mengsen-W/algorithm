struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MODULO: i32 = 1_000_000_007;
        let mut odd = 0;
        let mut even = 1;
        let mut subarrays = 0;
        let mut total = 0;

        for &x in arr.iter() {
            total += x;
            if total % 2 == 0 {
                subarrays = (subarrays + odd) % MODULO;
                even += 1;
            } else {
                subarrays = (subarrays + even) % MODULO;
                odd += 1;
            }
        }

        subarrays % MODULO
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 5], 4),
        (vec![2, 4, 6], 0),
        (vec![1, 2, 3, 4, 5, 6, 7], 16),
        (vec![100, 100, 99, 99], 4),
        (vec![7], 1),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::num_of_subarrays(nums), ans);
    }
}
