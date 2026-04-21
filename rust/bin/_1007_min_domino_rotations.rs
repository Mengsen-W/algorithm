struct Solution;

impl Solution {
    pub fn check(x: i32, tops: &[i32], bottoms: &[i32], n: usize) -> i32 {
        let (mut rotations_a, mut rotations_b) = (0, 0);
        for i in 0..n {
            if tops[i] != x && bottoms[i] != x {
                return -1;
            } else if tops[i] != x {
                rotations_a += 1;
            } else if bottoms[i] != x {
                rotations_b += 1;
            }
        }
        rotations_a.min(rotations_b)
    }

    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let n = tops.len();
        let rotations = Self::check(tops[0], &tops, &bottoms, n);
        if rotations != -1 || tops[0] == bottoms[0] {
            return rotations;
        }
        Self::check(bottoms[0], &tops, &bottoms, n)
    }
}

fn main() {
    let tests = vec![
        (vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2], 2),
        (vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4], -1),
    ];

    for (a, b, ans) in tests {
        assert_eq!(Solution::min_domino_rotations(a, b), ans);
    }
}
