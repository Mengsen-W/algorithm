struct Solution;

impl Solution {
    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut position = position;
        position.sort();
        let mut left = 1;
        let mut right = position[position.len() - 1] - position[0];
        let mut ans = -1;
        while left <= right {
            let mid = (left + right) / 2;
            if Self::check(mid, &position, m) {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }

    fn check(x: i32, position: &Vec<i32>, m: i32) -> bool {
        let mut pre = position[0];
        let mut cnt = 1;
        for &pos in &position[1..] {
            if pos - pre >= x {
                pre = pos;
                cnt += 1;
            }
        }
        cnt >= m
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 7], 3, 3),
        (vec![5, 4, 3, 2, 1, 1000000000], 2, 999999999),
    ];

    for (position, m, ans) in tests {
        assert_eq!(Solution::max_distance(position, m), ans);
    }
}
