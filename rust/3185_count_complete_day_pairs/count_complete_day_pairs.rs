struct Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut cnt = vec![0; 24];
        for hour in hours {
            ans += cnt[(24 - hour % 24) as usize % 24] as i64;
            cnt[hour as usize % 24] += 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![12, 12, 30, 24, 24], 2), (vec![72, 48, 24, 3], 3)];

    for (hours, ans) in tests {
        assert_eq!(Solution::count_complete_day_pairs(hours), ans);
    }
}
