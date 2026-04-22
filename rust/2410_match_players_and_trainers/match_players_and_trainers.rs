struct Solution;

impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort();
        trainers.sort();
        let mut j = 0;
        let n = trainers.len();
        for (i, &p) in players.iter().enumerate() {
            while j < n && trainers[j] < p {
                j += 1;
            }
            if j == n {
                return i as i32;
            }
            j += 1;
        }
        players.len() as i32
    }
}

fn main() {
    let tests = vec![
        (vec![4, 7, 9], vec![8, 2, 5, 8], 2),
        (vec![1, 1, 1], vec![10], 1),
    ];

    for (players, trainers, ans) in tests {
        assert_eq!(Solution::match_players_and_trainers(players, trainers), ans);
    }
}
