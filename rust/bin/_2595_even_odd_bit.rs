struct Solution;

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let mut res = vec![0, 0];
        let mut i = 0;
        let mut n_mut = n;
        while n_mut > 0 {
            res[i as usize] += n_mut & 1;
            n_mut >>= 1;
            i ^= 1;
        }
        res
    }
}

fn main() {
    let tests = vec![(50, vec![1, 2]), (2, vec![0, 1])];

    for (n, ans) in tests {
        assert_eq!(Solution::even_odd_bit(n), ans);
    }
}
