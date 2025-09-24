struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        use std::cmp::{max, min};
        let mut ans = 0;
        let n = arr.len();
        let mut sum = vec![0; 1001];
        for j in 0..n {
            for k in j + 1..n {
                if (arr[j] - arr[k]).abs() <= b {
                    let lj = arr[j] - a;
                    let rj = arr[j] + a;
                    let lk = arr[k] - c;
                    let rk = arr[k] + c;
                    let l = max(0, max(lj, lk));
                    let r = min(1000, min(rj, rk));
                    if l <= r {
                        if l == 0 {
                            ans += sum[r as usize];
                        } else {
                            ans += sum[r as usize] - sum[(l - 1) as usize];
                        }
                    }
                }
            }
            for k in arr[j]..=1000 {
                sum[k as usize] += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![3, 0, 1, 1, 9, 7], 7, 2, 3, 4),
        (vec![1, 1, 2, 2, 3], 0, 0, 1, 0),
    ];

    for (t, (arr, a, b, c, expected)) in tests.into_iter().enumerate() {
        assert_eq!(
            Solution::count_good_triplets(arr, a, b, c),
            expected,
            "t: {}",
            t
        );
    }
}
