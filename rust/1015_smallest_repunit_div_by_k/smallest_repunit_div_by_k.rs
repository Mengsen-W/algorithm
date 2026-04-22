struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }
        let mut temp = 1;
        let mut len = 1;
        while temp % k != 0 {
            temp = temp % k;
            temp = temp * 10 + 1;
            len += 1;
        }
        len as i32
    }
}

fn main() {
    let tests = vec![(1, 1), (2, -1), (3, 3)];

    for (k, expected) in tests {
        assert_eq!(Solution::smallest_repunit_div_by_k(k), expected);
    }
}
