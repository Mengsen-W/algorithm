/*
 * @Date: 2022-11-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-19
 * @FilePath: /algorithm/1732_largest_altitude/largest_altitude.rs
 */

pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let (mut ans, mut sum) = (0, 0);
    for x in gain {
        sum += x;
        ans = ans.max(sum);
    }
    ans
}

fn main() {
    {
        let gain = vec![-5, 1, 5, 0, -7];
        let ans = 1;
        assert_eq!(largest_altitude(gain), ans);
    }
    {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        let ans = 0;
        assert_eq!(largest_altitude(gain), ans);
    }
}
