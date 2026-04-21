struct Solution;

impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        let n = (r as f64).sqrt() as usize;
        let mut v = vec![0; n + 1];
        let mut res = r - l + 1;

        for i in 2..=n {
            if v[i] == 0 {
                let square = (i * i) as i32;
                if square >= l && square <= r {
                    res -= 1;
                }
                for j in (i * 2..=n).step_by(i) {
                    v[j] = 1;
                }
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(5, 7, 3), (4, 16, 11)];

    for (l, r, ans) in tests {
        assert_eq!(Solution::non_special_count(l, r), ans);
    }
}
