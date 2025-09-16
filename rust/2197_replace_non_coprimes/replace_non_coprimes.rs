struct Solution;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for mut num in nums {
            while let Some(&last) = ans.last() {
                let g = Self::gcd(last, num);
                if g > 1 {
                    num = last / g * num;
                    ans.pop();
                } else {
                    break;
                }
            }
            ans.push(num);
        }
        ans
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
    let tests = vec![
        (vec![6, 4, 3, 2, 7, 6, 2], vec![12, 7, 6]),
        (vec![2, 2, 1, 1, 3, 3, 3], vec![2, 1, 1, 3]),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::replace_non_coprimes(nums), expected);
    }
}
