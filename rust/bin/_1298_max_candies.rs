struct Solution;

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        use std::collections::VecDeque;
        let n = status.len();
        let mut can_open = vec![false; n];
        let mut has_box = vec![false; n];
        let mut used = vec![false; n];

        for i in 0..n {
            can_open[i] = status[i] == 1;
        }
        let mut q = VecDeque::new();
        let mut ans = 0;
        for box_id in initial_boxes {
            has_box[box_id as usize] = true;
            if can_open[box_id as usize] {
                q.push_back(box_id);
                used[box_id as usize] = true;
                ans += candies[box_id as usize];
            }
        }

        while let Some(big_box) = q.pop_front() {
            for &key in &keys[big_box as usize] {
                can_open[key as usize] = true;
                if !used[key as usize] && has_box[key as usize] {
                    q.push_back(key);
                    used[key as usize] = true;
                    ans += candies[key as usize];
                }
            }
            for &box_id in &contained_boxes[big_box as usize] {
                has_box[box_id as usize] = true;
                if !used[box_id as usize] && can_open[box_id as usize] {
                    q.push_back(box_id);
                    used[box_id as usize] = true;
                    ans += candies[box_id as usize];
                }
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 0, 1, 0],
            vec![7, 5, 4, 100],
            vec![vec![], vec![], vec![1], vec![]],
            vec![vec![1, 2], vec![3], vec![], vec![]],
            vec![0],
            16,
        ),
        (
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1],
            vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
            vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
            vec![0],
            6,
        ),
        (
            vec![1, 1, 1],
            vec![100, 1, 100],
            vec![vec![], vec![0, 2], vec![]],
            vec![vec![], vec![], vec![]],
            vec![1],
            1,
        ),
        (vec![1], vec![100], vec![vec![]], vec![vec![]], vec![], 0),
        (
            vec![1, 1, 1],
            vec![2, 3, 2],
            vec![vec![], vec![], vec![]],
            vec![vec![], vec![], vec![]],
            vec![2, 1, 0],
            7,
        ),
    ];

    for (candies, keys, contained_boxes, big_boxes, keys_to_open, expected) in tests {
        assert_eq!(
            Solution::max_candies(candies, keys, contained_boxes, big_boxes, keys_to_open),
            expected
        );
    }
}
