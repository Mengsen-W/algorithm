/*
 * @Date: 2022-09-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-08
 * @FilePath: /algorithm/667_construct_array/construct_array.rs
 */

pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
    let mut ans = (1..n + 1).collect::<Vec<_>>();
    let mut d = k;

    for i in 1..=k as usize {
        if i % 2 == 1 {
            ans[i] = ans[i - 1] + d;
        } else {
            ans[i] = ans[i - 1] - d;
        }

        d -= 1;
    }

    ans
}

fn main() {
    {
        let n = 3;
        let k = 1;
        let ans = vec![1, 2, 3];
        assert_eq!(construct_array(n, k), ans);
    }

    {
        let n = 3;
        let k = 2;
        let ans = vec![1, 3, 2];
        assert_eq!(construct_array(n, k), ans);
    }
}
