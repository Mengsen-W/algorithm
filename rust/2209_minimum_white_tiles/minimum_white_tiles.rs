struct Solution;

impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        use std::cmp::min;
        use std::mem::swap;
        let num_carpets = num_carpets as usize;
        let carpet_len = carpet_len as usize;
        let floor: Vec<_> = floor.chars().collect();
        let n = floor.len();
        let inf = i32::MAX / 2;
        let mut d = vec![inf; n + 1];
        let mut f = vec![inf; n + 1];

        d[0] = 0;
        for i in 1..=n {
            d[i] = d[i - 1] + if floor[i - 1] == '1' { 1 } else { 0 };
        }

        for _ in 1..=num_carpets {
            f[0] = 0;
            for i in 1..=n {
                f[i] = f[i - 1] + if floor[i - 1] == '1' { 1 } else { 0 };
                let p = if i >= carpet_len { i - carpet_len } else { 0 };
                f[i] = min(f[i], d[p]);
            }
            swap(&mut f, &mut d);
        }
        d[n]
    }
}

fn main() {
    let tests = vec![("10110101", 2, 2, 2), ("11111", 2, 3, 0)];

    for (floor, num_carpets, carpet_len, ans) in tests {
        assert_eq!(
            Solution::minimum_white_tiles(floor.to_string(), num_carpets, carpet_len),
            ans
        );
    }
}
