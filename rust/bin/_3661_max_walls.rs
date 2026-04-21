struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        use std::cmp;
        let n = robots.len();

        let mut robot_dist: Vec<(i32, i32)> =
            robots.into_iter().zip(distance.into_iter()).collect();
        robot_dist.sort_by_key(|&(pos, _)| pos);

        let mut walls = walls;
        walls.sort();
        let m = walls.len();

        let mut right_ptr = 0;
        let mut left_ptr = 0;
        let mut cur_ptr = 0;
        let mut robot_ptr = 0;

        let mut prev_right = 0;
        let mut sub_left = 0;
        let mut sub_right = 0;

        for i in 0..n {
            let (robot_pos, robot_dist_val) = robot_dist[i];
            while right_ptr < m && walls[right_ptr] <= robot_pos {
                right_ptr += 1;
            }

            let pos1 = right_ptr;
            while cur_ptr < m && walls[cur_ptr] < robot_pos {
                cur_ptr += 1;
            }
            let pos2 = cur_ptr;

            let mut left_bound = robot_pos - robot_dist_val;
            if i >= 1 {
                left_bound = cmp::max(robot_pos - robot_dist_val, robot_dist[i - 1].0 + 1);
            }

            while left_ptr < m && walls[left_ptr] < left_bound {
                left_ptr += 1;
            }
            let left_pos = left_ptr;
            let current_left = (pos1 - left_pos) as i32;

            let mut right_bound = robot_pos + robot_dist_val;
            if i < n - 1 {
                right_bound = cmp::min(robot_pos + robot_dist_val, robot_dist[i + 1].0 - 1);
            }

            while right_ptr < m && walls[right_ptr] <= right_bound {
                right_ptr += 1;
            }
            let right_pos = right_ptr;
            let current_right = (right_pos - pos2) as i32;

            let current_num = if i > 0 {
                while robot_ptr < m && walls[robot_ptr] < robot_dist[i - 1].0 {
                    robot_ptr += 1;
                }
                let pos3 = robot_ptr;
                (pos1 - pos3) as i32
            } else {
                0
            };

            if i == 0 {
                sub_left = current_left;
                sub_right = current_right;
            } else {
                let new_sub_left = cmp::max(
                    sub_left + current_left,
                    sub_right - prev_right + cmp::min(current_left + prev_right, current_num),
                );
                let new_sub_right = cmp::max(sub_left + current_right, sub_right + current_right);
                sub_left = new_sub_left;
                sub_right = new_sub_right;
            }

            prev_right = current_right;
        }

        cmp::max(sub_left, sub_right)
    }
}

fn main() {
    let tests = vec![
        (vec![4], vec![3], vec![1, 10], 1),
        (vec![10, 2], vec![5, 1], vec![5, 2, 7], 3),
        (vec![1, 2], vec![100, 1], vec![10], 0),
    ];

    for (robot, distance, walls, expected) in tests {
        assert_eq!(expected, Solution::max_walls(robot, distance, walls));
    }
}
