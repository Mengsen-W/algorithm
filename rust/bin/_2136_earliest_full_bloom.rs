/*
 * @Date: 2023-09-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-30
 * @FilePath: /algorithm/rust/2136_earliest_full_bloom/earliest_full_bloom.rs
 */

struct Solution;

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut id: Vec<usize> = (0..grow_time.len()).collect();
        id.sort_unstable_by(|&i, &j| grow_time[j].cmp(&grow_time[i]));
        let mut ans = 0;
        let mut days = 0;
        for &i in &id {
            days += plant_time[i]; // 累加播种天数
            ans = ans.max(days + grow_time[i]); // 再加上生长天数，就是这个种子的开花时间
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 4, 3], vec![2, 3, 1], 9),
        (vec![1, 2, 3, 2], vec![2, 1, 2, 1], 9),
        (vec![1], vec![1], 2),
    ];

    for (plant_time, grow_time, ans) in tests {
        assert_eq!(Solution::earliest_full_bloom(plant_time, grow_time), ans);
    }
}
