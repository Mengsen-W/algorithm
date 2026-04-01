struct Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by_key(|&i| positions[i]);

        let dir_chars: Vec<char> = directions.chars().collect();
        let mut stack: Vec<(usize, i32, char)> = Vec::new();

        for &i in &idx {
            let mut cur_idx = i;
            let mut cur_hp = healths[i];
            let mut cur_dir = dir_chars[i];

            while let Some(&(prev_idx, prev_hp, prev_dir)) = stack.last() {
                if prev_dir == 'R' && cur_dir == 'L' {
                    stack.pop();
                    if prev_hp > cur_hp {
                        cur_idx = prev_idx;
                        cur_hp = prev_hp - 1;
                        cur_dir = prev_dir;
                    } else if prev_hp < cur_hp {
                        cur_hp -= 1;
                    } else {
                        cur_idx = usize::MAX;
                        break;
                    }
                } else {
                    break;
                }
            }

            if cur_idx != usize::MAX {
                stack.push((cur_idx, cur_hp, cur_dir));
            }
        }

        stack.sort_by_key(|&(idx, _, _)| idx);
        stack.into_iter().map(|(_, hp, _)| hp).collect()
    }
}

fn main() {
    let tests = vec![
        (
            vec![5, 4, 3, 2, 1],
            vec![2, 17, 9, 15, 10],
            "RRRRR",
            vec![2, 17, 9, 15, 10],
        ),
        (vec![3, 5, 2, 6], vec![10, 10, 15, 12], "RLRL", vec![14]),
        (vec![1, 2, 5, 6], vec![10, 10, 11, 11], "RLRL", vec![]),
    ];

    for (positions, healths, directions, expected) in tests {
        assert_eq!(
            Solution::survived_robots_healths(positions, healths, directions.to_string()),
            expected
        );
    }
}
