/*
 * @Date: 2023-04-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-15
 * @FilePath: /algorithm/rust/1042_garden_no_adj/garden_no_adj.rs
 */

pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut path = vec![vec![]; n as usize];
    for p in &paths {
        path[p[0] as usize - 1].push(p[1] as usize - 1);
        path[p[1] as usize - 1].push(p[0] as usize - 1);
    }
    let mut queue = VecDeque::new();
    let mut res = vec![0; n as usize];
    for i in 0..n as usize {
        if res[i] == 0 {
            queue.push_back(i);
            while !queue.is_empty() {
                let len = queue.len();
                for _ in 0..len {
                    let cur = queue.pop_front().unwrap();
                    let mut temp = vec![0; 4];
                    //应该用哪种颜色
                    for &arround in &path[cur] {
                        if res[arround] == 0 {
                            queue.push_back(arround);
                        } else {
                            temp[res[arround] as usize - 1] = 1;
                        }
                    }
                    for i in 0..4 {
                        if temp[i] == 0 {
                            res[cur] = i as i32 + 1;
                            break;
                        }
                    }
                }
            }
        }
    }
    res
}

fn main() {
    {
        let n = 3;
        let paths = vec![[1, 2], vec![2, 3], vec![3, 1]];
        let ans = vec![1, 2, 3];
        assert_eq!(garden_no_adj(n, paths), ans);
    }

    {
        let n = 4;
        let paths = vec![[1, 2], vec![3, 4]];
        let ans = vec![1, 2, 1, 2];
        assert_eq!(garden_no_adj(n, paths), ans);
    }

    {
        let n = 4;
        let paths = vec![
            [1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 1],
            vec![1, 3],
            vec![2, 4],
        ];
        let ans = vec![1, 2, 3, 4];
        assert_eq!(garden_no_adj(n, paths), ans);
    }
}
