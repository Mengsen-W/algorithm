struct Solution;

impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut score = score.clone();
        score.sort_by(|u, v| v[k as usize].cmp(&u[k as usize]));
        score
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
            2,
            vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]],
        ),
        (
            vec![vec![3, 4], vec![5, 6]],
            0,
            vec![vec![5, 6], vec![3, 4]],
        ),
    ];

    for (score, k, ans) in tests {
        assert_eq!(Solution::sort_the_students(score, k), ans);
    }
}
