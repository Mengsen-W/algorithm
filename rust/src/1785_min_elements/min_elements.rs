/*
 * @Date: 2022-12-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-16
 * @FilePath: /algorithm/1785_min_elements/min_elements.rs
 */

pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
    ((((goal as i64) - nums.iter().map(|&x| x as i64).sum::<i64>()).abs() + (limit as i64) - 1)
        / (limit as i64)) as i32
}

fn main() {
    {
        let nums = vec![1, -1, 1];
        let limit = 3;
        let goal = -4;
        let ans = 2;
        assert_eq!(min_elements(nums, limit, goal), ans);
    }

    {
        let nums = vec![1, -10, 9, 1];
        let limit = 100;
        let goal = 0;
        let ans = 1;
        assert_eq!(min_elements(nums, limit, goal), ans);
    }
}
