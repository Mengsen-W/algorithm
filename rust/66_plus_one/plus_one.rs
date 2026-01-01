struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        for i in (0..=n - 1).rev() {
            digits[i] += 1;
            if digits[i] / 10 == 0 {
                return digits;
            }
            digits[i] = digits[i] % 10
        }
        digits.insert(0, 1);
        digits
    }
}

fn main() {
    let tests = vec![
        (vec![ 1, 2, 3 ], vec![ 1, 2, 4 ] ),
        (vec![ 1, 2, 9 ], vec![ 1, 3, 0 ] ),
    ];

    for (digits, ans) in tests {
        assert_eq!(Solution::plus_one(digits), ans);
    }
}
