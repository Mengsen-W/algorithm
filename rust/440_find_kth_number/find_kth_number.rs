struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        fn get_steps(curr: i32, n: i32) -> i32 {
            let n = n as i64;
            let mut steps = 0;
            let (mut first, mut last) = (curr as i64, curr as i64);
            while first <= n {
                steps += last.min(n) - first + 1;
                first = first * 10;
                last = last * 10 + 9;
            }
            steps as i32
        }
        let mut curr = 1;
        let mut k = k - 1;
        while k > 0 {
            let steps = get_steps(curr, n);
            if steps <= k {
                k -= steps;
                curr += 1;
            } else {
                curr = curr * 10;
                k -= 1;
            }
        }
        curr
    }
}

fn main() {
    let tests = vec![(13, 2, 10), (1, 1, 1)];

    for (n, k, expected) in tests {
        assert_eq!(Solution::find_kth_number(n, k), expected);
    }
}
