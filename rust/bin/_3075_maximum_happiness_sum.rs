struct Solution;

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut happiness = happiness;
        happiness.sort_by(|a, b| b.cmp(a));

        let mut ans: i64 = 0;
        for i in 0..(k as usize) {
            let val = happiness[i] as i64 - i as i64;
            ans += val.max(0);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], 2, 4),
        (vec![1, 1, 1, 1], 2, 1),
        (vec![2, 3, 4, 5], 1, 5),
    ];

    for (happiness, k, expected) in tests {
        assert_eq!(Solution::maximum_happiness_sum(happiness, k), expected);
    }
}
