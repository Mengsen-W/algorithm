struct Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 1..hours.len() {
            for j in 0..i {
                if (hours[i] + hours[j]) % 24 == 0 {
                    ans += 1;
                }
            }
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
