struct Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let (mut idx, mut pre, mut ret, mut n) = (0, -1, 0, n);
        while n > 0 {
            if n & 1 == 1 {
                if pre != -1 {
                    ret = ret.max(idx - pre);
                }
                pre = idx;
            }
            idx += 1;
            n >>= 1;
        }
        ret
    }
}

fn main() {
    let tests = vec![(22, 2), (8, 0), (5, 2)];

    for (n, ans) in tests {
        assert_eq!(Solution::binary_gap(n), ans);
    }
}
