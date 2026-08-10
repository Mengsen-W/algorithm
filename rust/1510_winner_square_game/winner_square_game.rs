struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut f = vec![false; n + 1];
        for i in 1..=n {
            let mut k = 1;
            while k * k <= i {
                if !f[i - k * k] {
                    f[i] = true;
                    break;
                }
                k += 1;
            }
        }

        f[n]
    }
}

fn main() {
    let tests = vec![(1, true), (2, false), (7, false), (17, false)];

    for (n, expected) in tests {
        assert_eq!(Solution::winner_square_game(n), expected);
    }
}
