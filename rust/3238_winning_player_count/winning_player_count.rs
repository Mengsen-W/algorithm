struct Solution;

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![vec![0; 11]; n as usize];
        for p in pick {
            cnt[p[0] as usize][p[1] as usize] += 1;
        }
        (0..n as usize)
            .filter(|&i| (0..=10).any(|j| cnt[i][j] > i))
            .count() as i32
    }
}

fn main() {
    let tests = vec![
        (
            4,
            vec![
                vec![0, 0],
                vec![1, 0],
                vec![1, 0],
                vec![2, 1],
                vec![2, 1],
                vec![2, 0],
            ],
            2,
        ),
        (5, vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4]], 0),
        (5, vec![vec![1, 1], vec![2, 4], vec![2, 4], vec![2, 4]], 1),
    ];

    for (n, pick, ans) in tests {
        assert_eq!(Solution::winning_player_count(n, pick), ans);
    }
}
