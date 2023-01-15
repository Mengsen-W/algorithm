/*
 * @Date: 2023-01-15
 * @LastEditors: error: git config user.email & please set dead value or install git
 * @LastEditTime: 2023-01-15
 * @FilePath: /algorithm/2239_min_max_game/min_max_game.rs
 */

pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
    while nums.len() > 1 {
        nums = nums
            .chunks(2)
            .enumerate()
            .map(|(i, v)| match i % 2 {
                0 => *v.iter().min().unwrap(),
                _ => *v.iter().max().unwrap(),
            })
            .collect();
    }

    nums[0]
}

fn main() {
    {
        let nums = vec![1, 3, 5, 2, 4, 8, 2, 2];
        let ans = 1;
        assert_eq!(min_max_game(nums), ans);
    }

    {
        let nums = vec![3];
        let ans = 3;
        assert_eq!(min_max_game(nums), ans);
    }
}
