struct Solution;

impl Solution {
    pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
        fn get_trend(x: i32, y: i32) -> i32 {
            if x == y {
                return 0;
            }
            return if x < y { -1 } else { 1 };
        }

        let n = temperature_a.len();
        let mut ans = 0;
        let mut cur = 0;
        for i in 1..n {
            let ta = get_trend(temperature_a[i - 1], temperature_a[i]);
            let tb = get_trend(temperature_b[i - 1], temperature_b[i]);
            if ta == tb {
                cur += 1;
                ans = ans.max(cur);
            } else {
                cur = 0;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![21, 18, 18, 18, 31], vec![34, 32, 16, 16, 17], 2),
        (
            vec![5, 10, 16, -6, 15, 11, 3],
            vec![16, 22, 23, 23, 25, 3, -16],
            3,
        ),
    ];

    for (temperature_a, temperature_b, ans) in tests {
        assert_eq!(
            Solution::temperature_trend(temperature_a, temperature_b),
            ans
        );
    }
}
