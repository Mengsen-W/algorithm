/*
 * @Date: 2022-07-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-28
 * @FilePath: /algorithm/1331_array_rank_transform/array_rank_transform.rs
 */

pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }
    let mut a = arr
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i))
        .collect::<Vec<_>>();
    a.sort_unstable();
    let mut ans = vec![0; a.len()];
    ans[a[0].1] = 1;
    a.windows(2).for_each(|x| {
        ans[x[1].1] = ans[x[0].1] + if x[0].0 == x[1].0 { 0 } else { 1 };
    });
    ans
}

fn main() {
    {
        let arr = vec![40, 10, 20, 30];
        let ans = vec![4, 1, 2, 3];
        assert_eq!(array_rank_transform(arr), ans);
    }

    {
        let arr = vec![100, 100, 100];
        let ans = vec![1, 1, 1];
        assert_eq!(array_rank_transform(arr), ans);
    }

    {
        let arr = vec![37, 12, 28, 9, 100, 56, 80, 5, 12];
        let ans = vec![5, 3, 4, 2, 8, 6, 7, 1, 3];
        assert_eq!(array_rank_transform(arr), ans);
    }
}
