struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let (mut ans, mut sum) = (0, 0);
        for x in gain {
            sum += x;
            ans = ans.max(sum);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![-5, 1, 5, 0, -7], 1),
        (vec![-4, -3, -2, -1, 4, 3, 2], 0),
    ];

    for (gain, expected) in tests {
        assert_eq!(Solution::largest_altitude(gain), expected);
    }
}
