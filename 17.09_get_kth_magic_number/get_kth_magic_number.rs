/*
 * @Date: 2022-09-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-28
 * @FilePath: /algorithm/17.09_get_kth_magic_number/get_kth_magic_number.rs
 */

pub fn get_kth_magic_number(k: i32) -> i32 {
    let mut nums = vec![0; k as usize];
    nums[0] = 1;
    let mut i3 = 0;
    let mut i5 = 0;
    let mut i7 = 0;
    for i in 1..(k as usize) {
        nums[i] = (nums[i3] * 3).min(nums[i5] * 5).min(nums[i7] * 7);
        if nums[i] == nums[i3] * 3 {
            i3 += 1;
        }
        if nums[i] == nums[i5] * 5 {
            i5 += 1;
        }
        if nums[i] == nums[i7] * 7 {
            i7 += 1;
        }
    }
    nums[(k - 1) as usize]
}

fn main() {
    assert_eq!(get_kth_magic_number(5), 9);
}
