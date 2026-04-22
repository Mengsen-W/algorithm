/*
 * @Date: 2023-03-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-15
 * @FilePath: /algorithm/rust/1615_maximal_network_rank/maximal_network_rank.rs
 */

pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let (mut connected, mut cnt, mut ret) = (
        vec![vec![0; n as usize]; n as usize],
        vec![0; n as usize],
        0,
    );
    for road in roads {
        connected[road[0] as usize][road[1] as usize] = 1;
        connected[road[1] as usize][road[0] as usize] = 1;
        cnt[road[0] as usize] += 1;
        cnt[road[1] as usize] += 1;
    }
    for i in 0..n as usize {
        for j in i + 1..n as usize {
            ret = ret.max(cnt[i] + cnt[j] - connected[i][j]);
        }
    }
    ret
}

fn main() {
    {
        let n = 4;
        let roads = [[0, 1], [0, 3], [1, 2], [1, 3]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = 4;
        assert_eq!(maximal_network_rank(n, roads), ans);
    }

    {
        let n = 5;
        let roads = [[0, 1], [0, 3], [1, 2], [1, 3], [2, 3], [2, 4]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = 5;
        assert_eq!(maximal_network_rank(n, roads), ans);
    }

    {
        let n = 8;
        let roads = [[0, 1], [1, 2], [2, 3], [2, 4], [5, 6], [5, 7]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = 5;
        assert_eq!(maximal_network_rank(n, roads), ans);
    }
}
