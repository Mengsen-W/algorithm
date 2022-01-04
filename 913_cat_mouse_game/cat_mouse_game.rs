/*
 * @Date: 2022-01-04 01:32:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-04 02:37:55
 */

struct Solution;

impl Solution {
    const MOUSE_WIN: i32 = 1;
    const CAT_WIN: i32 = 2;
    const DRAW: i32 = 0;
    const MAXN: usize = 51;
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut dp = vec![vec![vec![-1; Self::MAXN * 2]; Self::MAXN]; Self::MAXN];

        Self::get_result(1, 2, 0, &n, &mut dp, &graph)
    }
    fn get_result(
        mouse: usize,
        cat: usize,
        turns: usize,
        n: &usize,
        dp: &mut Vec<Vec<Vec<i32>>>,
        graph: &Vec<Vec<i32>>,
    ) -> i32 {
        if turns == *n * 2 {
            return Self::DRAW;
        }
        if dp[mouse][cat][turns] < 0 {
            if mouse == 0 {
                dp[mouse][cat][turns] = Self::MOUSE_WIN;
            } else if cat == mouse {
                dp[mouse][cat][turns] = Self::CAT_WIN;
            } else {
                Self::get_next_result(mouse, cat, turns, n, dp, graph);
            }
        }
        return dp[mouse][cat][turns];
    }

    fn get_next_result(
        mouse: usize,
        cat: usize,
        turns: usize,
        n: &usize,
        dp: &mut Vec<Vec<Vec<i32>>>,
        graph: &Vec<Vec<i32>>,
    ) {
        let cur_move = if turns % 2 == 0 { mouse } else { cat };
        let default_result = if cur_move == mouse {
            Self::CAT_WIN
        } else {
            Self::MOUSE_WIN
        };
        let mut result = default_result;
        for next in graph[cur_move].iter() {
            if cur_move == cat && *next == 0 {
                continue;
            }
            let next_mouse = if cur_move == mouse {
                *next as usize
            } else {
                mouse
            };
            let next_cat = if cur_move == cat { *next as usize } else { cat };
            let next_result = Self::get_result(next_mouse, next_cat, turns + 1, n, dp, graph);
            if next_result != default_result {
                result = next_result;
                if result != Self::DRAW {
                    break;
                }
            }
        }
        dp[mouse][cat][turns] = result;
    }
}

fn main() {
    assert_eq!(
        Solution::cat_mouse_game(vec![
            vec![2, 5],
            vec![3],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![2, 3],
            vec![0, 2, 3]
        ]),
        0
    );

    assert_eq!(
        Solution::cat_mouse_game(vec![vec![1, 3], vec![0], vec![3], vec![0, 2]]),
        1
    );

    assert_eq!(
        Solution::cat_mouse_game(vec![
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
            vec![1, 3, 6, 10, 13, 19, 23, 28]
        ]),
        1
    );
}
