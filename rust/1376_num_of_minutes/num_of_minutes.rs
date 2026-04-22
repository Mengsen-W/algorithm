/*
 * @Date: 2023-05-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-01
 * @FilePath: /algorithm/rust/1376_num_of_minutes/num_of_minutes.rs
 */

pub fn num_of_minutes(n: i32, _head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    fn dfs(cur: i32, manager: &Vec<i32>, inform_time: &Vec<i32>, visited: &mut Vec<i32>) -> i32 {
        if cur == -1 {
            return 0;
        }
        let cur = cur as usize;
        if manager[cur] == -1 {
            visited[cur] = 0;
            return inform_time[cur];
        }
        if visited[cur] != i32::MAX {
            return visited[cur] + inform_time[cur];
        }
        visited[cur] = dfs(manager[cur], manager, inform_time, visited);
        visited[cur] + inform_time[cur]
    }
    let mut visited = vec![i32::MAX; n as usize];
    (0..n).fold(0, |ans, i| {
        ans.max(dfs(i, &manager, &inform_time, &mut visited))
    })
}

fn main() {
    {
        let n = 1;
        let head_id = 0;
        let manager = vec![-1];
        let inform_time = vec![0];
        let ans = 0;
        assert_eq!(num_of_minutes(n, head_id, manager, inform_time), ans);
    }

    {
        let n = 5;
        let head_id = 2;
        let manager = vec![2, 2, -1, 2, 2, 2];
        let inform_time = vec![0, 0, 1, 0, 0, 0];
        let ans = 1;
        assert_eq!(num_of_minutes(n, head_id, manager, inform_time), ans);
    }
}
