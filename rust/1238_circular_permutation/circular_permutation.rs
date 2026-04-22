/*
 * @Date: 2023-02-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-23
 * @FilePath: /algorithm/rust/1238_circular_permutation/circular_permutation.rs
 */

pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
    (0..1 << n).map(|i| (i >> 1) ^ i ^ start).collect()
}

fn main() {
    {
        let n = 2;
        let start = 3;
        let ans = vec![3, 2, 0, 1];
        assert_eq!(circular_permutation(n, start), ans);
    }

    {
        let n = 3;
        let start = 2;
        let ans = vec![2, 3, 1, 0, 4, 5, 7, 6];
        assert_eq!(circular_permutation(n, start), ans);
    }
}
