struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let fp = Self::fingerprint(n);
        (0..31)
            .map(|x| Self::fingerprint(1i32 << x))
            .any(|x| x == fp)
    }

    fn fingerprint(n: i32) -> [i32; 10] {
        let mut m: [i32; 10] = [0; 10];
        n.to_string()
            .as_bytes()
            .iter()
            .for_each(|&x| m[x as usize - '0' as usize] += 1);
        return m;
    }
}

fn main() {
    assert_eq!(Solution::reordered_power_of2(1), true);
    assert_eq!(Solution::reordered_power_of2(10), false);
    assert_eq!(Solution::reordered_power_of2(16), true);
    assert_eq!(Solution::reordered_power_of2(24), false);
    assert_eq!(Solution::reordered_power_of2(46), true);
    assert_eq!(Solution::reordered_power_of2(8208), false);
    assert_eq!(Solution::reordered_power_of2(9474), false);
    assert_eq!(Solution::reordered_power_of2(15), false);
    assert_eq!(Solution::reordered_power_of2(32), true);
    assert_eq!(Solution::reordered_power_of2(0), true);
    assert_eq!(Solution::reordered_power_of2(2147483647), false);
}
