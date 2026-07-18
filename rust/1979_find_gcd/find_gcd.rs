struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mx = *nums.iter().max().unwrap();
        let mn = *nums.iter().min().unwrap();
        Self::gcd(mx, mn)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

fn main() {
    let testes = vec![
        (vec![2, 5, 6, 9, 10], 2),
        (vec![7, 5, 6, 8, 3], 1),
        (vec![3, 3], 3),
    ];

    for (nums, expected) in testes {
        assert_eq!(Solution::find_gcd(nums), expected);
    }
}
