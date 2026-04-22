/*
 * @Date: 2023-02-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-10
 * @FilePath: /algorithm/rust/1223_die_simulator/die_simulator.rs
 */

pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
    const M: i32 = 100000007;
    let n = n as usize;
    let mut d = vec![[0; 6]; n + 1];
    let mut sum = vec![0; n + 1];
    sum[0] = 1;
    for i in 1..=n {
        for j in 0..6 {
            let pos = 0.max(i as i32 - roll_max[j] - 1) as usize;
            let sub = ((sum[pos] - d[pos][j]) % M + M) % M;
            d[i][j] = ((sum[i - 1] - sub) % M + M) % M;
            if i as i32 <= roll_max[j] {
                d[i][j] = (d[i][j] + 1) % M;
            }
            sum[i] = (sum[i] + d[i][j]) % M;
        }
    }
    sum[n]
}

fn main() {
    {
        let n = 2;
        let roll_max = vec![1, 1, 2, 2, 2, 3];
        let ans = 34;
        assert_eq!(die_simulator(n, roll_max), ans);
    }

    {
        let n = 2;
        let roll_max = vec![1, 1, 1, 1, 1, 1];
        let ans = 30;
        assert_eq!(die_simulator(n, roll_max), ans);
    }

    {
        let n = 3;
        let roll_max = vec![1, 1, 1, 2, 2, 3];
        let ans = 181;
        assert_eq!(die_simulator(n, roll_max), ans);
    }
}
