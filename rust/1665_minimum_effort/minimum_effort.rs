struct Solution;

impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_by(|a, b| (b[1] - b[0]).cmp(&(a[1] - a[0])));
        let mut ans = 0;
        let mut remain = 0;
        for task in tasks.iter() {
            if remain <= task[1] {
                ans += task[1] - remain;
            }
            remain = std::cmp::max(task[1] - task[0], remain - task[0]);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 2], vec![2, 4], vec![4, 8]], 8),
        (
            vec![
                vec![1, 3],
                vec![2, 4],
                vec![10, 11],
                vec![10, 12],
                vec![8, 9],
            ],
            32,
        ),
        (
            vec![
                vec![1, 7],
                vec![2, 8],
                vec![3, 9],
                vec![4, 10],
                vec![5, 11],
                vec![6, 12],
            ],
            27,
        ),
    ];

    for (tasks, expected) in tests {
        assert_eq!(Solution::minimum_effort(tasks), expected);
    }
}
