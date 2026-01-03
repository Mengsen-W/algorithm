struct Solution;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mod_val: i64 = 1000000007;
        let mut fi0: i64 = 6;
        let mut fi1: i64 = 6;

        for _ in 2..=n {
            let new_fi0 = (2 * fi0 + 2 * fi1) % mod_val;
            let new_fi1 = (2 * fi0 + 3 * fi1) % mod_val;
            fi0 = new_fi0;
            fi1 = new_fi1;
        }

        ((fi0 + fi1) % mod_val) as i32
    }
}

fn main() {
    let tests = vec![(1, 12), (2, 54), (3, 246), (7, 106494), (5000, 30228214)];

    for (n, ans) in tests {
        assert_eq!(Solution::num_of_ways(n), ans);
    }
}
