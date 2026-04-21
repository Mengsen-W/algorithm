struct Solution;

impl Solution {
    pub fn network_delay_time_dijkstra(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut arcs: Vec<Vec<i32>> = vec![vec![10001; n + 1]; n + 1];
        for i in 1..=n {
            arcs[i][i] = 0;
        }

        for time in times.iter() {
            arcs[time[0] as usize][time[1] as usize] = time[2];
        }
        let mut dist = arcs[k].clone();
        let mut flag = vec![false; n + 1];
        flag[k] = true;
        for _ in 1..=n {
            let mut minn = 10010;
            let mut pos = 0;
            for j in 1..=n {
                if !flag[j] && dist[j] < minn {
                    pos = j;
                    minn = dist[j];
                }
            }
            flag[pos] = true;
            for j in 1..=n {
                if !flag[j] && dist[pos] + arcs[pos][j] < dist[j] {
                    dist[j] = dist[pos] + arcs[pos][j];
                }
            }
        }
        let mut ret = i32::MIN;
        for i in 1..dist.len() {
            if dist[i] > ret {
                ret = dist[i]
            }
        }
        match ret == 10001 {
            true => -1,
            false => ret,
        }
    }

    pub fn network_delay_time_floyd(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut arcs: Vec<Vec<i32>> = vec![vec![10010; n + 1]; n + 1];
        for i in 1..=n {
            arcs[i][i] = 0;
        }

        for time in times.iter() {
            arcs[time[0] as usize][time[1] as usize] = time[2];
        }

        for r in 1..=n {
            for i in 1..=n {
                for j in 1..=n {
                    if arcs[i][j] > arcs[i][r] + arcs[r][j] {
                        arcs[i][j] = arcs[i][r] + arcs[r][j];
                    }
                }
            }
        }
        let mut ret = i32::MIN;
        for i in 1..arcs[k].len() {
            if arcs[k][i] > ret {
                ret = arcs[k][i]
            }
        }
        match ret == 10010 {
            true => -1,
            false => ret,
        }
    }
}

fn main() {
    let tests = vec![
        (vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2, 2),
        (vec![vec![1, 2, 1]], 2, 1, 1),
        (vec![vec![1, 2, 1]], 2, 2, -1),
    ];

    for (times, n, k, ans) in tests {
        assert_eq!(
            Solution::network_delay_time_dijkstra(times.clone(), n, k),
            ans
        );
        assert_eq!(Solution::network_delay_time_floyd(times.clone(), n, k), ans);
    }
}
