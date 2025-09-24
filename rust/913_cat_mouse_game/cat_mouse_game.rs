struct Solution;

#[derive(Clone, std::cmp::PartialEq, Copy)]
enum State {
    NoFound,
    Draw,
    CatWin,
    MouseWin,
}

impl Solution {
    fn get_result(
        mouse: usize,
        cat: usize,
        dp: &mut Vec<Vec<Vec<State>>>,
        turn: usize,
        n: usize,
        g: &Vec<Vec<i32>>,
    ) -> State {
        if turn == 2 * n {
            return State::Draw;
        }
        if dp[mouse][cat][turn] == State::NoFound {
            match mouse {
                0 => dp[mouse][cat][turn] = State::MouseWin,
                i if i == cat => dp[mouse][cat][turn] = State::CatWin,
                _ => dp[mouse][cat][turn] = Self::dfs(dp, mouse, cat, turn, g, n),
            }
        }
        dp[mouse][cat][turn]
    }
    fn dfs(
        dp: &mut Vec<Vec<Vec<State>>>,
        mouse: usize,
        cat: usize,
        turn: usize,
        g: &Vec<Vec<i32>>,
        n: usize,
    ) -> State {
        let is_mouse = turn % 2 == 0;
        let cur_move = if is_mouse { mouse } else { cat };
        let default_val = if is_mouse {
            State::CatWin
        } else {
            State::MouseWin
        };
        let mut val = default_val;
        for ele in g[cur_move].iter().filter(|&&x| x != 0 || is_mouse) {
            let r = if is_mouse {
                Self::get_result(*ele as usize, cat, dp, turn + 1, n, g)
            } else {
                Self::get_result(mouse, *ele as usize, dp, turn + 1, n, g)
            };
            if r != default_val {
                val = r;
                if r != State::Draw {
                    break;
                }
            }
        }
        val
    }
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let len = graph.len();
        let mut dp = vec![vec![vec![State::NoFound; 2 * len]; len]; len];
        match Self::get_result(1, 2, &mut dp, 0, len, &graph) {
            State::CatWin => 2,
            State::Draw => 0,
            State::MouseWin => 1,
            State::NoFound => panic!("no found"),
        }
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![2, 5],
                vec![3],
                vec![0, 4, 5],
                vec![1, 4, 5],
                vec![2, 3],
                vec![0, 2, 3],
            ],
            0,
        ),
        (vec![vec![1, 3], vec![0], vec![3], vec![0, 2]], 1),
        (
            vec![
                vec![5, 21, 28],
                vec![6, 8, 9, 13, 23, 24, 30],
                vec![9, 10, 22, 24],
                vec![24, 30],
                vec![5, 6, 8, 9, 13, 18, 19, 20, 24],
                vec![0, 4, 9, 10, 11, 12, 22, 27],
                vec![1, 4, 9, 11, 16, 19, 25, 30],
                vec![8, 9, 13, 19, 25, 26],
                vec![1, 4, 7, 9, 29],
                vec![1, 2, 4, 5, 6, 7, 8, 13, 18, 19, 24, 26, 28, 29],
                vec![2, 5, 15, 22, 27, 30],
                vec![5, 6, 12, 24],
                vec![5, 11, 20, 22, 23],
                vec![1, 4, 7, 9, 29, 30],
                vec![19, 24, 27],
                vec![10, 16, 19],
                vec![6, 15, 27],
                vec![20, 22, 24, 29],
                vec![4, 9, 21],
                vec![4, 6, 7, 9, 14, 15, 20, 26, 28, 30],
                vec![4, 12, 17, 19, 21],
                vec![0, 18, 20, 27],
                vec![2, 5, 10, 12, 17],
                vec![1, 12, 26, 30],
                vec![1, 2, 3, 4, 9, 11, 14, 17, 27, 29],
                vec![6, 7, 26, 27, 29],
                vec![7, 9, 19, 23, 25],
                vec![5, 10, 14, 16, 21, 24, 25],
                vec![0, 9, 19, 30],
                vec![8, 9, 13, 17, 24, 25],
                vec![1, 3, 6, 10, 13, 19, 23, 28],
            ],
            1,
        ),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::cat_mouse_game(nums), ans);
    }
}
