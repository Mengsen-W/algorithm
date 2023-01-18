/*
 * @Date: 2022-06-28
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-28
 * @FilePath: /algorithm/324_wiggle_sort/wiggle_sort.rs
 */

pub fn wiggle_sort(nums: &mut Vec<i32>) {
    nums.sort_unstable();
    let back_n = nums.len() / 2;
    let front_n = nums.len() - back_n;
    let front = nums.get(0..front_n).unwrap().to_vec();
    let back = nums.get(front_n..).unwrap().to_vec();

    for i in 0..nums.len() {
        if i & 1 == 0 {
            nums[i] = front[front_n - 1 - i / 2];
        } else {
            nums[i] = back[back_n - 1 - i / 2];
        }
    }
}

fn main() {
    {
        let mut nums: Vec<i32> = vec![1, 5, 1, 1, 6, 4];
        let ans = vec![1, 6, 1, 5, 1, 4];
        wiggle_sort(&mut nums);
        assert_eq!(nums, ans);
    }

    {
        let mut nums: Vec<i32> = vec![1, 3, 2, 2, 3, 1];
        let ans = vec![2, 3, 1, 3, 1, 2];
        wiggle_sort(&mut nums);
        assert_eq!(nums, ans);
    }
}
