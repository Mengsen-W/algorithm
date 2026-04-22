struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let week_number = n / 7;
        let first_week_money = (1 + 7) * 7 / 2;
        let last_week_money = first_week_money + 7 * (week_number - 1);
        let week_money = (first_week_money + last_week_money) * week_number / 2;
        let day_number = n % 7;
        let first_day_money = 1 + week_number;
        let last_day_money = first_day_money + day_number - 1;
        let day_money = (first_day_money + last_day_money) * day_number / 2;
        week_money + day_money
    }
}

fn main() {
    let tests = vec![(4, 10), (10, 37), (20, 96)];

    for (n, expected) in tests {
        assert_eq!(Solution::total_money(n), expected);
    }
}
