struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut x, mut y) = (0, 0);
        let mut dir = 0;

        let mut obstacle_set: HashSet<i32> = HashSet::new();
        for obstacle in obstacles {
            obstacle_set.insert(obstacle[0] * 60001 + obstacle[1]);
        }

        let mut max_distance = 0;

        for &cmd in &commands {
            if cmd == -1 {
                dir = (dir + 1) % 4;
            } else if cmd == -2 {
                dir = (dir + 3) % 4;
            } else {
                for _ in 0..cmd {
                    let nx = x + dirs[dir].0;
                    let ny = y + dirs[dir].1;
                    if obstacle_set.contains(&(nx * 60001 + ny)) {
                        break;
                    }

                    x = nx;
                    y = ny;
                    max_distance = max_distance.max(x * x + y * y);
                }
            }
        }

        max_distance
    }
}

fn main() {
    let tests = vec![
        (vec![4, -1, 3], vec![vec![]], 25),
        (vec![4, -1, 4, -2, 4], vec![vec![2, 4]], 65),
    ];

    for (commands, obstacles, expected) in tests {
        assert_eq!(Solution::robot_sim(commands, obstacles), expected);
    }
}
