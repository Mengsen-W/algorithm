/*
 * @Date: 2023-05-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-13
 * @FilePath: /algorithm/rust/2441_find_max_k/find_max_k.rs
 */

pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let n: usize = nums.len();
    let (mut pre_index, mut back_index): (usize, usize) = (0, n - 1);
    while pre_index < back_index {
        while pre_index < back_index && nums[pre_index] < -nums[back_index] {
            pre_index += 1;
        }
        if nums[pre_index] == -nums[back_index] {
            return nums[back_index];
        }
        back_index -= 1;
    }
    -1
}

fn main() {
    {
        let nums = vec![-1, -2, -3, 3];
        let ans = 3;
        assert_eq!(find_max_k(nums), ans);
    }

    {
        let nums = vec![-1, 10, 6, 7, -7, 1];
        let ans = 7;
        assert_eq!(find_max_k(nums), ans);
    }

    {
        let nums = vec![-10, 8, 6, 7, -2, -3];
        let ans = -1;
        assert_eq!(find_max_k(nums), ans);
    }
}
