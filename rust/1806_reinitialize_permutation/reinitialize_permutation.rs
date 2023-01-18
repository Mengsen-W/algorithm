/*
 * @Date: 2023-01-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-09
 * @FilePath: /algorithm/1806_reinitialize_permutation/reinitialize_permutation.rs
 */

pub fn reinitialize_permutation(n: i32) -> i32 {
    if n == 2 {
        return 1;
    }
    let (mut step, mut pow2) = (1, 2);
    while pow2 != 1 {
        step += 1;
        pow2 = pow2 * 2 % (n - 1);
    }
    step
}

fn main() {
    {
        let n = 2;
        let ans = 1;
        assert_eq!(reinitialize_permutation(n), ans);
    }

    {
        let n = 4;
        let ans = 2;
        assert_eq!(reinitialize_permutation(n), ans);
    }

    {
        let n = 6;
        let ans = 4;
        assert_eq!(reinitialize_permutation(n), ans);
    }
}
