struct Solution;

impl Solution {
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        use std::cmp::max;
        let n = values.len();
        let mut g: Vec<Vec<(i32, i32)>> = vec![vec![]; n];
        for edge in edges {
            g[edge[0] as usize].push((edge[1], edge[2]));
            g[edge[1] as usize].push((edge[0], edge[2]));
        }

        let mut visited = vec![false; n];
        visited[0] = true;
        let mut ans = 0;
        fn dfs(
            u: usize,
            time: i32,
            value: i32,
            ans: &mut i32,
            g: &Vec<Vec<(i32, i32)>>,
            visited: &mut Vec<bool>,
            values: &Vec<i32>,
            max_time: &i32,
        ) {
            if u == 0 {
                *ans = max(*ans, value);
            }
            for (v, dist) in &g[u] {
                if time + dist <= *max_time {
                    if !visited[*v as usize] {
                        visited[*v as usize] = true;
                        dfs(
                            *v as usize,
                            time + dist,
                            value + values[*v as usize],
                            ans,
                            g,
                            visited,
                            values,
                            max_time,
                        );
                        visited[*v as usize] = false;
                    } else {
                        dfs(
                            *v as usize,
                            time + dist,
                            value,
                            ans,
                            g,
                            visited,
                            values,
                            max_time,
                        );
                    }
                }
            }
        }

        dfs(
            0,
            0,
            values[0],
            &mut ans,
            &g,
            &mut visited,
            &values,
            &max_time,
        );
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![0, 32, 10, 43],
            vec![vec![0, 1, 10], vec![1, 2, 15], vec![0, 3, 10]],
            49,
            75,
        ),
        (
            vec![5, 10, 15, 20],
            vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 3, 10]],
            30,
            25,
        ),
        (
            vec![1, 2, 3, 4],
            vec![
                vec![0, 1, 10],
                vec![1, 2, 11],
                vec![2, 3, 12],
                vec![1, 3, 13],
            ],
            50,
            7,
        ),
        (vec![0, 1, 2], vec![vec![1, 2, 10]], 10, 0),
    ];

    for (values, edges, max_time, ans) in tests {
        assert_eq!(Solution::maximal_path_quality(values, edges, max_time), ans);
    }
}
