struct Solution;

struct Answer {
    ans: [i32; 3],
}

impl Answer {
    fn new() -> Self {
        Answer { ans: [0, 0, 0] }
    }

    fn put(&mut self, x: i32) {
        if x > self.ans[0] {
            self.ans[2] = self.ans[1];
            self.ans[1] = self.ans[0];
            self.ans[0] = x;
        } else if x != self.ans[0] && x > self.ans[1] {
            self.ans[2] = self.ans[1];
            self.ans[1] = x;
        } else if x != self.ans[0] && x != self.ans[1] && x > self.ans[2] {
            self.ans[2] = x;
        }
    }

    fn get(&self) -> Vec<i32> {
        let mut ret = Vec::new();
        for &num in &self.ans {
            if num != 0 {
                ret.push(num);
            }
        }
        ret
    }
}

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut sum1 = vec![vec![0; n + 2]; m + 1];
        let mut sum2 = vec![vec![0; n + 2]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                sum1[i][j] = sum1[i - 1][j - 1] + grid[i - 1][j - 1];
                sum2[i][j] = sum2[i - 1][j + 1] + grid[i - 1][j - 1];
            }
        }

        let mut ans = Answer::new();
        for i in 0..m {
            for j in 0..n {
                // 单独的一个格子也是菱形
                ans.put(grid[i][j]);
                let mut k = i + 2;
                while k < m {
                    let ux = i;
                    let uy = j;
                    let dx = k;
                    let dy = j;
                    let lx = (i + k) / 2;
                    let ly = j as i32 - ((k - i) / 2) as i32;
                    let rx = (i + k) / 2;
                    let ry = j + (k - i) / 2;

                    if ly < 0 || ry >= n {
                        break;
                    }

                    let sum = (sum2[lx + 1][(ly + 1) as usize] - sum2[ux][uy + 2])
                        + (sum1[rx + 1][ry + 1] - sum1[ux][uy])
                        + (sum1[dx + 1][dy + 1] - sum1[lx][ly as usize])
                        + (sum2[dx + 1][dy + 1] - sum2[rx][ry + 2])
                        - (grid[ux][uy] + grid[dx][dy] + grid[lx][ly as usize] + grid[rx][ry]);

                    ans.put(sum);
                    k += 2;
                }
            }
        }

        ans.get()
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![3, 4, 5, 1, 3],
                vec![3, 3, 4, 2, 3],
                vec![20, 30, 200, 40, 10],
                vec![1, 5, 5, 4, 1],
                vec![4, 3, 2, 2, 5],
            ],
            vec![228, 216, 211],
        ),
        (
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![20, 9, 8],
        ),
        (vec![vec![7, 7, 7]], vec![7]),
    ];

    for (grid, expected) in tests {
        assert_eq!(Solution::get_biggest_three(grid), expected);
    }
}
