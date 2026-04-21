/*
 * @Date: 2023-04-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-17
 * @FilePath: /algorithm/rust/2409_count_days_together/count_days_together.rs
 */

pub fn count_days_together(
    arrive_alice: String,
    leave_alice: String,
    arrive_bob: String,
    leave_bob: String,
) -> i32 {
    fn get_value(date: &[u8]) -> i32 {
        let (month, day) = (
            (date[0] - b'0') as i32 * 10 + (date[1] - b'0') as i32,
            (date[3] - b'0') as i32 * 10 + (date[4] - b'0') as i32,
        );
        let days = (1..month).fold(0, |days, i| {
            days + [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][i as usize - 1]
        });
        days + day
    }
    let (alice, bob) = (
        [
            get_value(arrive_alice.as_bytes()),
            get_value(leave_alice.as_bytes()),
        ],
        [
            get_value(arrive_bob.as_bytes()),
            get_value(leave_bob.as_bytes()),
        ],
    );
    if alice[0] > bob[1] || bob[0] > alice[1] {
        return 0;
    }
    alice[1].min(bob[1]) - alice[0].max(bob[0]) + 1
}

fn main() {
    {
        let arrive_alice = "08-15".to_string();
        let leave_alice = "08-18".to_string();
        let arrive_bob = "08-16".to_string();
        let leave_bob = "08-19".to_string();
        let ans = 3;
        assert_eq!(
            count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob),
            ans
        );
    }

    {
        let arrive_alice = "10-01".to_string();
        let leave_alice = "10-31".to_string();
        let arrive_bob = "11-01".to_string();
        let leave_bob = "12-31".to_string();
        let ans = 0;
        assert_eq!(
            count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob),
            ans
        );
    }
}
