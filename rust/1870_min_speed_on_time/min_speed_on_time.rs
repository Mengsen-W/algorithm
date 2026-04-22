struct Solution;

impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let n = dist.len();
        // 将 hour 乘 100 以转为整数
        let hr = (hour * 100.0).round() as i64;
        // 时间必须要大于路程段数减 1
        if hr <= ((n as i64 - 1) * 100) {
            return -1;
        }
        // 二分
        let mut l = 1;
        let mut r = 10_000_000;
        while l < r {
            let mid = l + (r - l) / 2;
            // 判断当前时速是否满足时限
            let mut t = 0i64;
            // 前 n-1 段中第 i 段贡献的时间： floor(dist[i] / mid)
            for i in 0..n - 1 {
                t += (dist[i] as i64 - 1) / mid as i64 + 1;
            }
            // 最后一段贡献的时间： dist[n-1] / mid
            t *= mid as i64;
            t += dist[n - 1] as i64;
            if t * 100 <= hr * mid as i64 {
                // 通分以转化为整数比较
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l // 满足条件的最小时速
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 2], 6.0, 1),
        (vec![1, 3, 2], 2.7, 3),
        (vec![1, 3, 2], 1.9, -1),
    ];

    for (dist, hour, ans) in tests {
        assert_eq!(Solution::min_speed_on_time(dist, hour), ans);
    }
}
