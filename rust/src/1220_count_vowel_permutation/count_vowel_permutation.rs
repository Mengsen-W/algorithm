/*
 * @Date: 2022-01-17 07:21:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-17 07:50:40
 */

struct Solution;

type Mat = Vec<Vec<i64>>;
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let factor: Mat = vec![
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 0, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0],
        ];
        let res: Mat = Self::fast_pow(factor, n as i64 - 1, MOD);
        let mut ans: i64 = 0;
        for i in 0..5 {
            ans += res[i].iter().sum::<i64>();
            ans %= MOD;
        }
        ans as i32
    }

    fn fast_pow(matrix: Mat, n: i64, mod_: i64) -> Mat {
        let m = matrix.len();
        let mut res: Mat = vec![vec![0; m]; m];
        let mut curr: Mat = matrix;
        for i in 0..m {
            res[i][i] = 1;
        }
        let mut i = n;
        while i != 0 {
            if i & 1 == 1 {
                res = Self::multiply(&curr, &res, mod_);
            }
            curr = Self::multiply(&curr, &curr, mod_);
            i >>= 1;
        }
        res
    }

    fn multiply(matrix_a: &Mat, matrix_b: &Mat, mod_: i64) -> Mat {
        let m = matrix_a.len();
        let n = matrix_b[0].len();
        let mut res: Mat = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                for k in 0..matrix_a[0].len() {
                    res[i][j] = (res[i][j] + matrix_a[i][k] * matrix_b[k][j]) % mod_;
                }
            }
        }
        res
    }
}

fn main() {
    assert_eq!(Solution::count_vowel_permutation(1), 5);
    assert_eq!(Solution::count_vowel_permutation(2), 10);
}
