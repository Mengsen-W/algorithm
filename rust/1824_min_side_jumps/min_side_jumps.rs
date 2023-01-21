/*
 * @Date: 2023-01-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-21
 * @FilePath: /algorithm/rust/1824_min_side_jumps/min_side_jumps.rs
 */

pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
    const INF: i32 = 0x3f3f3f3f;
    let n = obstacles.len() - 1;
    let mut d = vec![1, 0, 1];
    for i in 1..=n {
        let mut min_cnt = INF;
        for j in 0..3 {
            if j == obstacles[i] - 1 {
                d[j as usize] = INF;
            } else {
                min_cnt = min_cnt.min(d[j as usize]);
            }
        }
        for j in 0..3 {
            if j == obstacles[i] - 1 {
                continue;
            }
            d[j as usize] = (min_cnt + 1).min(d[j as usize]);
        }
    }
    *d.iter().min().unwrap()
}

fn main() {
    {
        let obstacles = vec![0, 1, 2, 3, 0];
        let ans = 2;
        assert_eq!(min_side_jumps(obstacles), ans);
    }

    {
        let obstacles = vec![0, 1, 1, 3, 3, 0];
        let ans = 0;
        assert_eq!(min_side_jumps(obstacles), ans);
    }

    {
        let obstacles = vec![0, 2, 1, 0, 3, 0];
        let ans = 2;
        assert_eq!(min_side_jumps(obstacles), ans);
    }
}
