struct Solution;

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        // 二分查找下界与上界
        let mut l = 1i64;
        let mut r = total_trips as i64 * *time.iter().max().unwrap() as i64;
        // 二分查找寻找满足要求的最小的 t
        while l < r {
            let mid = l + (r - l) / 2;
            if Self::check(mid, &time, total_trips) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }

    // 判断 t 时间内是否可以完成 totalTrips 趟旅途
    pub fn check(t: i64, time: &Vec<i32>, total_trips: i32) -> bool {
        let mut cnt = 0i64;
        for &period in time {
            cnt += t / period as i64;
        }
        cnt >= total_trips as i64
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3], 5, 3), (vec![2], 1, 2)];

    for (time, total_trips, ans) in tests {
        assert_eq!(Solution::minimum_time(time, total_trips), ans);
    }
}
