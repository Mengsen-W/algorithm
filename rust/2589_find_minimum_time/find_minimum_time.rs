/*
 * @Date: 2024-05-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-15
 * @FilePath: /algorithm/rust/2589_find_minimum_time/find_minimum_time.rs
 */

struct Solution;

impl Solution {
    pub fn find_minimum_time(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks = tasks;
        tasks.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut stack = vec![[-1, -1, 0]];
        for task in tasks.iter() {
            let start = task[0];
            let end = task[1];
            let mut duration = task[2];
            let k = Self::binary_search(&stack, start);
            duration -= stack.last().unwrap()[2] - stack[k - 1][2];
            if start <= stack[k - 1][1] {
                duration -= stack[k - 1][1] - start + 1;
            }
            if duration <= 0 {
                continue;
            }
            while end - stack.last().unwrap()[1] <= duration {
                duration += stack.last().unwrap()[1] - stack.last().unwrap()[0] + 1;
                stack.pop();
            }
            stack.push([end - duration + 1, end, stack.last().unwrap()[2] + duration]);
        }
        stack.last().unwrap()[2]
    }

    fn binary_search(stack: &Vec<[i32; 3]>, target: i32) -> usize {
        let mut low = 0;
        let mut high = stack.len();
        while low < high {
            let mid = low + (high - low) / 2;
            if stack[mid][0] > target {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}

fn main() {
    let tests = vec![
        (vec![vec![2, 3, 1], vec![4, 5, 1], vec![1, 5, 2]], 2),
        (vec![vec![1, 3, 2], vec![2, 5, 3], vec![5, 6, 2]], 4),
    ];

    for (tasks, ans) in tests {
        assert_eq!(Solution::find_minimum_time(tasks), ans);
    }
}
