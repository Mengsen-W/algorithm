/*
 * @Date: 2023-02-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-27
 * @FilePath: /algorithm/rust/1144_moves_to_make_zigzag/moves_to_make_zigzag.rs
 */

pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
    nums.get(nums.len() - 2)
        .map(|x| {
            std::iter::once([1001, nums[0], nums[1]].as_slice())
                .chain(nums.windows(3))
                .chain(std::iter::once([*x, nums[nums.len() - 1], 1001].as_slice()))
                .fold((0, 0), |s, x| (s.1 + 0.max(x[1] - x[0].min(x[2]) + 1), s.0))
        })
        .map(|x| x.0.min(x.1))
        .unwrap_or(0)
}

fn main() {
    {
        let nums = vec![1, 2, 3];
        let ans = 2;
        assert_eq!(moves_to_make_zigzag(nums), ans);
    }

    {
        let nums = vec![9, 6, 1, 6, 2];
        let ans = 4;
        assert_eq!(moves_to_make_zigzag(nums), ans);
    }
}
