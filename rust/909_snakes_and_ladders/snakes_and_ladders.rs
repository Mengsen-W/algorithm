/*
 * @Date: 2021-06-27 10:07:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-27 10:54:39
 */

fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    fn id2rc(id: i32, n: i32) -> (i32, i32) {
        let r = (id - 1) / n;
        let mut c = (id - 1) % n;
        if r % 2 == 1 {
            c = n - 1 - c;
        }
        (n - 1 - r, c)
    }

    let n = board.len();
    let mut vis: Vec<bool> = vec![false; n * n + 1];
    let mut q: std::collections::VecDeque<(i32, i32)> = std::collections::VecDeque::new();
    q.push_back((1, 0));
    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        for i in 1..=6 {
            let mut nxt = p.0 + i;
            if nxt > (n * n) as i32 {
                break;
            }
            let rc = id2rc(nxt, n as i32);
            if board[rc.0 as usize][rc.1 as usize] > 0 {
                nxt = board[rc.0 as usize][rc.1 as usize];
            }
            if nxt == (n * n) as i32 {
                return p.1 + 1;
            }
            if !vis[nxt as usize] {
                vis[nxt as usize] = true;
                q.push_back((nxt, p.1 + 1));
            }
        }
    }
    -1
}

fn main() {
    let board = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];
    assert_eq!(snakes_and_ladders(board), 4);
}
