struct Solution;

impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        fn pow_mod(mut x: i32, mut y: i32, mod_: i32) -> i32 {
            let mut res = 1;
            while y > 0 {
                if (y & 1) == 1 {
                    res = res * x % mod_;
                }
                x = x * x % mod_;
                y >>= 1;
            }
            res
        }
        let mut ans = Vec::new();
        for (i, v) in variables.iter().enumerate() {
            if pow_mod(pow_mod(v[0], v[1], 10), v[2], v[3]) == target {
                ans.push(i as i32);
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]],
            2,
            vec![0, 2],
        ),
        (vec![vec![39, 3, 1000, 1000]], 17, vec![]),
    ];

    for (variables, target, ans) in tests {
        assert_eq!(Solution::get_good_indices(variables, target), ans);
    }
}
