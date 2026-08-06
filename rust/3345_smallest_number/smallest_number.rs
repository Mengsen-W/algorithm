struct Solution;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        fn check(num: i32, t: i32) -> bool {
            let mut product = 1;
            let mut x = num;
            while x > 0 {
                product *= x % 10;
                x /= 10;
                if product == 0 {
                    break;
                }
            }
            product % t == 0
        }

        let mut cur = n;
        while !check(cur, t) {
            cur += 1;
        }
        cur
    }
}

fn main() {
    let tests = vec![(10, 2, 10), (15, 3, 16)];

    for (n, t, expected) in tests {
        assert_eq!(Solution::smallest_number(n, t), expected);
    }
}
