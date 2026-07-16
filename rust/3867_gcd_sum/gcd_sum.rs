struct Solution;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut mx = vec![0; n];
        let mut prefix_max = i32::MIN;
        for (i, &x) in nums.iter().enumerate() {
            prefix_max = prefix_max.max(x);
            mx[i] = prefix_max;
        }

        let mut prefix_gcd: Vec<i32> = nums
            .iter()
            .zip(mx.iter())
            .map(|(&x, &m)| Self::gcd(x, m))
            .collect();

        prefix_gcd.sort();
        let mut ans: i64 = 0;
        let mut left = 0;
        let mut right = n - 1;
        while left < right {
            ans += Self::gcd(prefix_gcd[left], prefix_gcd[right]) as i64;
            left += 1;
            right -= 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![2, 6, 4], 2), (vec![3, 6, 2, 8], 5)];

    for (test, expected) in tests {
        assert_eq!(Solution::gcd_sum(test), expected);
    }
}
