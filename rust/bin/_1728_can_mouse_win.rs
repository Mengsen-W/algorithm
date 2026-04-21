struct Solution;

impl Solution {
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        const MOUSE_TURN: i32 = 0;
        const CAT_TURN: i32 = 1;
        const UNKNOWN: i32 = 0;
        const MOUSE_WIN: i32 = 1;
        const CAT_WIN: i32 = 2;
        const MAX_MOVES: i32 = 1000;
        struct Meta {
            dirs: [[i32; 2]; 4],
            grid: Vec<Vec<char>>,
            rows: i32,
            cols: i32,
            cat_jump: i32,
            mouse_jump: i32,
            food: i32,
            degrees: [[[i32; 2]; 64]; 64],
            results: [[[[i32; 2]; 2]; 64]; 64],
        }

        fn get_pos(row: i32, col: i32, meta: &Meta) -> i32 {
            row * meta.cols + col
        }

        fn get_prev_states(mouse: i32, cat: i32, turn: i32, meta: &Meta) -> Vec<(i32, i32, i32)> {
            let mut prev_states: Vec<(i32, i32, i32)> = Vec::new();
            let (mouse_row, mouse_col) = (mouse / meta.cols, mouse % meta.cols);
            let (cat_row, cat_col) = (cat / meta.cols, cat % meta.cols);
            let prev_turn = if turn == MOUSE_TURN {
                CAT_TURN
            } else {
                MOUSE_TURN
            };
            let max_jump = if prev_turn == MOUSE_TURN {
                meta.mouse_jump
            } else {
                meta.cat_jump
            };
            let start_row = if prev_turn == MOUSE_TURN {
                mouse_row
            } else {
                cat_row
            };
            let start_col = if prev_turn == MOUSE_TURN {
                mouse_col
            } else {
                cat_col
            };

            prev_states.push((mouse, cat, prev_turn));

            for dir in meta.dirs {
                let (mut i, mut j) = (start_row + dir[0], start_col + dir[1]);
                let mut jump = 1;
                while i >= 0
                    && i < meta.rows
                    && j >= 0
                    && j < meta.cols
                    && meta.grid[i as usize][j as usize] != '#'
                    && jump <= max_jump
                {
                    let prev_mouse_row = if prev_turn == MOUSE_TURN {
                        i
                    } else {
                        mouse_row
                    };
                    let prev_mouse_col = if prev_turn == MOUSE_TURN {
                        j
                    } else {
                        mouse_col
                    };
                    let prev_cat_row = if prev_turn == MOUSE_TURN { cat_row } else { i };
                    let prev_cat_col = if prev_turn == MOUSE_TURN { cat_col } else { j };
                    let prev_mouse = get_pos(prev_mouse_row, prev_mouse_col, meta);
                    let prev_cat = get_pos(prev_cat_row, prev_cat_col, meta);
                    prev_states.push((prev_mouse, prev_cat, prev_turn));
                    i += dir[0];
                    j += dir[1];
                    jump += 1;
                }
            }
            prev_states
        }

        let mut meta = Meta {
            dirs: [[0, 1], [1, 0], [0, -1], [-1, 0]],
            grid: grid.iter().map(|s| s.chars().collect()).collect(),
            rows: grid.len() as i32,
            cols: grid[0].len() as i32,
            cat_jump: cat_jump,
            mouse_jump: mouse_jump,
            food: 0,
            degrees: [[[0; 2]; 64]; 64],
            results: [[[[0; 2]; 2]; 64]; 64],
        };

        let (mut start_mouse, mut start_cat) = (-1, -1);

        for i in 0..meta.rows {
            for j in 0..meta.cols {
                let c = meta.grid[i as usize][j as usize];
                if c == 'M' {
                    start_mouse = get_pos(i, j, &meta);
                } else if c == 'C' {
                    start_cat = get_pos(i, j, &meta);
                } else if c == 'F' {
                    meta.food = get_pos(i, j, &meta);
                }
            }
        }

        let total = meta.rows * meta.cols;
        let mut qu: Vec<(i32, i32, i32)> = Vec::new();

        for mouse in 0..total {
            let mouse_row = mouse / meta.cols;
            let mouse_col = mouse % meta.cols;
            if meta.grid[mouse_row as usize][mouse_col as usize] == '#' {
                continue;
            }
            for cat in 0..total {
                let cat_row = cat / meta.cols;
                let cat_col = cat % meta.cols;
                if meta.grid[cat_row as usize][cat_col as usize] == '#' {
                    continue;
                }
                meta.degrees[mouse as usize][cat as usize][MOUSE_TURN as usize] += 1;
                meta.degrees[mouse as usize][cat as usize][CAT_TURN as usize] += 1;
                for dir in meta.dirs {
                    let mut row = mouse_row + dir[0];
                    let mut col = mouse_col + dir[1];
                    let mut jump = 1;
                    while row >= 0
                        && row < meta.rows
                        && col >= 0
                        && col < meta.cols
                        && meta.grid[row as usize][col as usize] != '#'
                        && jump <= meta.mouse_jump
                    {
                        let next_mouse = get_pos(row, col, &meta);
                        let next_cat = get_pos(cat_row, cat_col, &meta);
                        meta.degrees[next_mouse as usize][next_cat as usize]
                            [MOUSE_TURN as usize] += 1;
                        row += dir[0];
                        col += dir[1];
                        jump += 1;
                    }
                    let mut row = cat_row + dir[0];
                    let mut col = cat_col + dir[1];
                    let mut jump = 1;
                    while row >= 0
                        && row < meta.rows
                        && col >= 0
                        && col < meta.cols
                        && meta.grid[row as usize][col as usize] != '#'
                        && jump <= meta.cat_jump
                    {
                        let next_mouse = get_pos(mouse_row, mouse_col, &meta);
                        let next_cat = get_pos(row, col, &meta);
                        meta.degrees[next_mouse as usize][next_cat as usize][CAT_TURN as usize] +=
                            1;
                        row += dir[0];
                        col += dir[1];
                        jump += 1;
                    }
                }
            }
        }
        // 猫和老鼠在同一个单元格，猫获胜
        for pos in 0..total {
            let row = pos / meta.cols;
            let col = pos % meta.cols;
            if meta.grid[row as usize][col as usize] == '#' {
                continue;
            }
            meta.results[pos as usize][pos as usize][MOUSE_TURN as usize][0] = CAT_WIN;
            meta.results[pos as usize][pos as usize][MOUSE_TURN as usize][1] = 0;
            meta.results[pos as usize][pos as usize][CAT_TURN as usize][0] = CAT_WIN;
            meta.results[pos as usize][pos as usize][CAT_TURN as usize][1] = 0;
            qu.push((pos, pos, MOUSE_TURN));
            qu.push((pos, pos, CAT_TURN));
        }
        // 猫和食物在同一个单元格，猫获胜
        for mouse in 0..total {
            let mouse_row = mouse / meta.cols;
            let mouse_col = mouse % meta.cols;
            if meta.grid[mouse_row as usize][mouse_col as usize] == '#' || mouse == meta.food {
                continue;
            }
            meta.results[mouse as usize][meta.food as usize][MOUSE_TURN as usize][0] = CAT_WIN;
            meta.results[mouse as usize][meta.food as usize][MOUSE_TURN as usize][1] = 0;
            meta.results[mouse as usize][meta.food as usize][CAT_TURN as usize][0] = CAT_WIN;
            meta.results[mouse as usize][meta.food as usize][CAT_TURN as usize][1] = 0;
            qu.push((mouse, meta.food, MOUSE_TURN));
            qu.push((mouse, meta.food, CAT_TURN));
        }
        // 老鼠和食物在同一个单元格且猫和食物不在同一个单元格，老鼠获胜
        for cat in 0..total {
            let cat_row = cat / meta.cols;
            let cat_col = cat % meta.cols;
            if meta.grid[cat_row as usize][cat_col as usize] == '#' || cat == meta.food {
                continue;
            }
            meta.results[meta.food as usize][cat as usize][MOUSE_TURN as usize][0] = MOUSE_WIN;
            meta.results[meta.food as usize][cat as usize][MOUSE_TURN as usize][1] = 0;
            meta.results[meta.food as usize][cat as usize][CAT_TURN as usize][0] = MOUSE_WIN;
            meta.results[meta.food as usize][cat as usize][CAT_TURN as usize][1] = 0;
            qu.push((meta.food, cat, MOUSE_TURN));
            qu.push((meta.food, cat, CAT_TURN));
        }
        // 拓扑排序
        while qu.len() > 0 {
            let (mouse, cat, turn) = qu.pop().unwrap();
            let result = meta.results[mouse as usize][cat as usize][turn as usize][0];
            let moves = meta.results[mouse as usize][cat as usize][turn as usize][1];
            let prev_states = get_prev_states(mouse, cat, turn, &meta);
            for (prev_mouse, prev_cat, prev_turn) in prev_states {
                if meta.results[prev_mouse as usize][prev_cat as usize][prev_turn as usize][0]
                    == UNKNOWN
                {
                    let can_win = (result == MOUSE_WIN && prev_turn == MOUSE_TURN)
                        || (result == CAT_WIN && prev_turn == CAT_TURN);
                    if can_win {
                        meta.results[prev_mouse as usize][prev_cat as usize][prev_turn as usize]
                            [0] = result;
                        meta.results[prev_mouse as usize][prev_cat as usize][prev_turn as usize]
                            [1] = moves + 1;
                        qu.push((prev_mouse, prev_cat, prev_turn));
                    } else {
                        meta.degrees[prev_mouse as usize][prev_cat as usize][prev_turn as usize] -=
                            1;
                        if meta.degrees[prev_mouse as usize][prev_cat as usize][prev_turn as usize]
                            == 0
                        {
                            let lose_result = if prev_turn == MOUSE_TURN {
                                CAT_WIN
                            } else {
                                MOUSE_WIN
                            };
                            meta.results[prev_mouse as usize][prev_cat as usize]
                                [prev_turn as usize][0] = lose_result;
                            meta.results[prev_mouse as usize][prev_cat as usize]
                                [prev_turn as usize][1] = moves + 1;
                            qu.push((prev_mouse, prev_cat, prev_turn));
                        }
                    }
                }
            }
        }
        return meta.results[start_mouse as usize][start_cat as usize][MOUSE_TURN as usize][0]
            == MOUSE_WIN
            && meta.results[start_mouse as usize][start_cat as usize][MOUSE_TURN as usize][1]
                <= MAX_MOVES;
    }
}

fn main() {
    let tests = vec![
        (vec!["####F", "#C...", "M...."], 1, 2, true),
        (vec!["M.C...F"], 1, 4, true),
        (vec!["M.C...F"], 1, 3, false),
        (vec!["C...#", "...#F", "....#", "M...."], 2, 5, false),
        (
            vec![".M...", "..#..", "#..#.", "C#.#.", "...#F"],
            1,
            1,
            true,
        ),
    ];

    for (grid, cat_jump, mouse_jump, ans) in tests {
        assert_eq!(
            Solution::can_mouse_win(
                grid.iter().map(|s| s.to_string()).collect(),
                cat_jump,
                mouse_jump
            ),
            ans
        );
    }
}
