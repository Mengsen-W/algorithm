struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut digit_base = 10i32.pow((num as f32).log10() as u32);
        let mut num = num;
        while digit_base > 0 {
            if (num / digit_base) % 10 == 6 {
                num += 3 * digit_base;
                return num;
            }
            digit_base /= 10;
        }

        num
    }
}

fn main() {
    let tests = vec![(9669, 9969), (9996, 9999), (9999, 9999)];

    for (input, expected) in tests {
        assert_eq!(Solution::maximum69_number(input), expected);
    }
}
