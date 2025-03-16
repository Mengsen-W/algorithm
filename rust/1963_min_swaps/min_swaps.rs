struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut cnt = 0;
        let mut mincnt = 0;
        for ch in s.chars() {
            if ch == '[' {
                cnt += 1;
            } else {
                cnt -= 1;
                mincnt = mincnt.min(cnt);
            }
        }
        (-mincnt + 1) / 2
    }
}

fn main() {
    let tests = vec![("][][", 1), ("]]][[[", 2), ("[]", 0)];

    for (s, ans) in tests {
        assert_eq!(Solution::min_swaps(s.to_string()), ans);
    }
}
