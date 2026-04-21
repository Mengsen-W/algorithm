/*
 * @Date: 2022-07-05
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-05
 * @FilePath: /algorithm/729_my_calendar/my_calendar.rs
 */

struct MyCalendar {
    events: Vec<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        Self { events: Vec::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let n = self.events.len();
        let mut lo = 0;
        let mut hi = n;

        while lo < hi && hi <= n {
            let mid = lo + (hi - lo) / 2;

            if start >= self.events[mid].1 {
                lo = mid + 1;
            } else if end <= self.events[mid].0 {
                hi = mid;
            } else {
                return false;
            }
        }

        self.events.insert(lo, (start, end));

        true
    }
}

fn main() {
    let mut my_calendar = MyCalendar::new();
    assert!(my_calendar.book(10, 20));
    assert!(!my_calendar.book(15, 25));
    assert!(my_calendar.book(20, 30));
}
