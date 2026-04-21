/*
 * @Date: 2023-06-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-07
 * @FilePath: /algorithm/rust/2611_mice_and_cheese/mice_and_cheese.rs
 */

pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
    let n: usize = reward1.len();
    let mut arr: Vec<(i32, usize)> = (0..n).map(|x| (reward1[x] - reward2[x], x)).collect();
    arr.sort_by(|a, b| b.0.cmp(&a.0));
    let k = k as usize;
    if k >= n {
        reward1.iter().sum::<i32>();
    }
    arr[0..k]
        .iter()
        .fold(reward2.iter().sum::<i32>(), |mut acc, x| {
            acc -= reward2[x.1];
            acc += reward1[x.1];
            acc
        })
}

fn main() {
    {
        let reward1 = vec![1, 1, 3, 4];
        let reward2 = vec![4, 4, 1, 1];
        let k = 2;
        let ans = 15;
        assert_eq!(mice_and_cheese(reward1, reward2, k), ans);
    }

    {
        let reward1 = vec![1, 1];
        let reward2 = vec![1, 1];
        let k = 2;
        let ans = 2;
        assert_eq!(mice_and_cheese(reward1, reward2, k), ans);
    }
}
