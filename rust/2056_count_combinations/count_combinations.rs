struct Solution;

// 定义车、象、后棋子的方向
const ROOK_DIRECTIONS: &[(i32, i32)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRECTIONS: &[(i32, i32)] = &[(1, 1), (1, -1), (-1, 1), (-1, -1)];
const QUEEN_DIRECTIONS: &[(i32, i32)] = &[
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

#[derive(Clone)]
struct Movement {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    dx: i32,
    dy: i32,
    cur_x: i32,
    cur_y: i32,
}

impl Movement {
    fn new(start_x: i32, start_y: i32, end_x: i32, end_y: i32, dx: i32, dy: i32) -> Self {
        Movement {
            start_x,
            start_y,
            end_x,
            end_y,
            dx,
            dy,
            cur_x: start_x,
            cur_y: start_y,
        }
    }

    fn reset(&mut self) {
        self.cur_x = self.start_x;
        self.cur_y = self.start_y;
    }

    fn stopped(&self) -> bool {
        self.cur_x == self.end_x && self.cur_y == self.end_y
    }

    fn advance(&mut self) {
        if !self.stopped() {
            self.cur_x += self.dx;
            self.cur_y += self.dy;
        }
    }

    fn cross(&mut self, oth: &mut Movement) -> bool {
        self.reset();
        oth.reset();
        while !self.stopped() || !oth.stopped() {
            self.advance();
            oth.advance();
            if self.cur_x == oth.cur_x && self.cur_y == oth.cur_y {
                return true;
            }
        }
        false
    }
}

impl Solution {
    pub fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
        let n = pieces.len();
        let mut res = 0;
        let mut stack = Vec::new();

        // 判断是否相交的函数，stack 需要是可变引用
        fn check(stack: &mut Vec<Movement>, u: usize) -> bool {
            for v in 0..u {
                // 分割 stack，避免同时借用同一对象
                let (left, right) = stack.split_at_mut(u); // 切分成两个部分
                if left[v].cross(&mut right[0]) {
                    // 比较时仅借用 stack 中非重叠部分
                    return false;
                }
            }
            true
        }

        // 深度优先搜索函数
        fn dfs(
            pieces: &Vec<String>,
            positions: &Vec<Vec<i32>>,
            n: usize,
            stack: &mut Vec<Movement>,
            res: &mut i32,
            u: usize,
        ) {
            if u == n {
                *res += 1;
                return;
            }

            let directions = match pieces[u].as_str() {
                "rook" => ROOK_DIRECTIONS,
                "queen" => QUEEN_DIRECTIONS,
                _ => BISHOP_DIRECTIONS,
            };

            // 处理第 u 个棋子原地不动的情况
            stack.push(Movement::new(
                positions[u][0],
                positions[u][1],
                positions[u][0],
                positions[u][1],
                0,
                0,
            ));
            if check(stack, u) {
                dfs(pieces, positions, n, stack, res, u + 1);
            }
            stack.pop();

            // 枚举第 u 个棋子在所有方向、所有步数的情况
            for dir in directions.iter() {
                for j in 1..8 {
                    let x = positions[u][0] + dir.0 * j;
                    let y = positions[u][1] + dir.1 * j;
                    if x < 1 || x > 8 || y < 1 || y > 8 {
                        break;
                    }
                    stack.push(Movement::new(
                        positions[u][0],
                        positions[u][1],
                        x,
                        y,
                        dir.0,
                        dir.1,
                    ));
                    if check(stack, u) {
                        dfs(pieces, positions, n, stack, res, u + 1);
                    }
                    stack.pop();
                }
            }
        }

        dfs(&pieces, &positions, n, &mut stack, &mut res, 0);
        res
    }
}

fn main() {
    let tests = vec![
        (vec!["rook"], vec![vec![1, 1]], 15),
        (vec!["queen"], vec![vec![1, 1]], 22),
        (vec!["bishop"], vec![vec![4, 3]], 12),
        (vec!["rook", "rook"], vec![vec![1, 1], vec![8, 8]], 223),
        (vec!["queen", "bishop"], vec![vec![5, 7], vec![3, 4]], 281),
    ];

    for (pieces, positions, ans) in tests {
        assert_eq!(
            Solution::count_combinations(pieces.iter().map(|s| s.to_string()).collect(), positions),
            ans
        );
    }
}
