struct Solution;

impl Solution {
    const MOD: i64 = 1000000007;
    pub fn count_good_numbers(n: i64) -> i32 {
        (Self::quickmul(5, (n + 1) / 2) * Self::quickmul(4, n / 2) % Self::MOD) as i32
    }

    // 快速幂求出 x^y % Self::MOD
    fn quickmul(x: i32, mut y: i64) -> i64 {
        let mut ret = 1 as i64;
        let mut mul = x as i64;
        while y > 0 {
            if y % 2 == 1 {
                ret = ret * mul % Self::MOD;
            }
            mul = mul * mul % Self::MOD;
            y /= 2;
        }
        ret
    }
}

fn main() {
    let tests = vec![(1, 5), (4, 400), (50, 564908303)];

    for (n, ans) in tests {
        assert_eq!(Solution::count_good_numbers(n), ans);
    }
}
