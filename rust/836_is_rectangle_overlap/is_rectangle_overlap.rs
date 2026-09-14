struct Solution;

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        std::cmp::min(rec1[2], rec2[2]) > std::cmp::max(rec1[0], rec2[0])
            && std::cmp::min(rec1[3], rec2[3]) > std::cmp::max(rec1[1], rec2[1])
    }
}

fn main() {
    let tests = vec![
        (vec![0, 0, 2, 2], vec![1, 1, 3, 3], true),
        (vec![0, 0, 1, 1], vec![1, 0, 2, 1], false),
        (vec![0, 0, 1, 1], vec![2, 2, 3, 3], false),
    ];

    for (rec1, rec2, ans) in tests {
        assert_eq!(Solution::is_rectangle_overlap(rec1, rec2), ans);
    }
}
