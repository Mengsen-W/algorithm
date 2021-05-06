/*
 * @Date: 2021-05-06 10:52:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-06 10:58:18
 */

fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let mut a = [0u16; 10001];
    nums.into_iter().for_each(|x| a[x as usize] += 1);
    a[2..]
        .iter()
        .zip(2..)
        .fold((a[1] as i32, a[0] as i32), |s, (&x, i)| {
            (s.0.max(s.1 + i * x as i32), s.0)
        })
        .0
}

fn main() {
    {
        let nums = vec![3, 4, 2];
        assert_eq!(delete_and_earn(nums), 6);
    }
    {
        let nums = vec![2, 2, 3, 3, 3, 4];
        assert_eq!(delete_and_earn(nums), 9);
    }
}
