struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut res = 0;
        for i in 0..n {
            if colors[i] != colors[(i + n - 1) % n] && colors[i] != colors[(i + 1) % n] {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![1, 1, 1], 0), (vec![0, 1, 0, 0, 1], 3)];

    for (colors, ans) in tests {
        assert_eq!(Solution::number_of_alternating_groups(colors), ans);
    }
}
