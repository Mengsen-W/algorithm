/*
 * @Date: 2021-04-19 08:38:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-19 09:11:06
 */

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[j] = nums[i];
            j += 1;
        }
    }
    j as i32
}

fn main() {
    {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(remove_element(&mut nums, 3), 2);
        println!("{:?}", nums);
    }
    {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element(&mut nums, 2), 5);
        println!("{:?}", nums);
    }
}
