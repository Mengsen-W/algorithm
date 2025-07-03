struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut ans = 0;
        let mut k = k;
        while k != 1 {
            let t = 31 - k.leading_zeros();
            let t = if (1 << t) == k { t - 1 } else { t };
            k -= 1 << t;
            ans += 1;
        }
        (b'a' + ans) as char
    }
}

fn main() {
    let tests = vec![(5, 'b'), (10, 'c')];

    for (k, ans) in tests {
        assert_eq!(Solution::kth_character(k), ans);
    }
}
