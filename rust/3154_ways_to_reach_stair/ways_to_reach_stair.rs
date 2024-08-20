struct Solution;

impl Solution {
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        let mut n = 0;
        let mut npow = 1;
        let mut ans = 0;
        loop {
            if npow - n - 1 <= k && k <= npow {
                ans += Self::comb(n + 1, npow - k);
            } else if npow - n - 1 > k {
                break;
            }
            n += 1;
            npow *= 2;
        }
        ans
    }

    fn comb(n: i32, k: i32) -> i32 {
        let mut ans: i64 = 1;
        for i in (n - k + 1..=n).rev() {
            ans *= i as i64;
            ans /= (n - i + 1) as i64;
        }
        ans as i32
    }
}

fn main() {
    let tests = vec![(0, 2), (1, 4)];

    for (k, ans) in tests {
        assert_eq!(Solution::ways_to_reach_stair(k), ans);
    }
}
