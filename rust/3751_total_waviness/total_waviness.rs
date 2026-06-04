struct Solution;

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        fn get_waviness(x: i32) -> i32 {
            let s = x.to_string();
            let chars: Vec<char> = s.chars().collect();
            let mut waviness = 0;

            for i in 1..chars.len() - 1 {
                let is_peak = chars[i] > chars[i - 1] && chars[i] > chars[i + 1];
                let is_valley = chars[i] < chars[i - 1] && chars[i] < chars[i + 1];
                if is_peak || is_valley {
                    waviness += 1;
                }
            }

            waviness
        }

        let mut total = 0;
        for i in num1..=num2 {
            total += get_waviness(i);
        }

        total
    }
}

fn main() {
    let tests = vec![(120, 130, 3), (198, 202, 3), (4848, 4848, 2)];

    for (num1, num2, expected) in tests {
        assert_eq!(Solution::total_waviness(num1, num2), expected);
    }
}
