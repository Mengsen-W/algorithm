struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 1..=9 {
            let mut num = i;
            for j in (i + 1)..=9 {
                num = num * 10 + j;
                if num >= low && num <= high {
                    ans.push(num);
                }
            }
        }
        ans.sort();
        ans
    }
}

fn main() {
    let tests = vec![
        (100, 300, vec![123, 234]),
        (1000, 13000, vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]),
    ];

    for (low, high, expected) in tests {
        assert_eq!(Solution::sequential_digits(low, high), expected);
    }
}
