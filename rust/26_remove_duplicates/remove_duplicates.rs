/*
 * @Date: 2021-04-18 09:49:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-18 10:15:20
 */

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    if len < 2 {
        return len as i32;
    }
    let mut left: usize = 0;
    for right in 1..len {
        if nums[left] != nums[right] {
            left += 1;
            nums[left] = nums[right];
        }
    }
    left += 1;
    nums.resize(left, 0);
    left as i32
}

fn main() {
    {
        let mut nums = vec![1, 1, 2];
        remove_duplicates(&mut nums);
        println!("{:?}", nums);
    }
    {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        remove_duplicates(&mut nums);
        println!("{:?}", nums);
    }
}
