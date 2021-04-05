/*
 * @Date: 2021-04-06 01:05:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-06 01:09:39
 */

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let size = nums.len();
    if size <= 2 {
        return size as i32;
    }
    let mut slow = 2;
    let mut fast = 2;
    while fast < size {
        if nums[slow - 2] != nums[fast] {
            nums[slow] = nums[fast];
            slow += 1;
        }
        fast += 1;
    }
    slow as i32
}

fn main() {
    {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut nums), 5);
        println!("{:?}", nums);
    }
    {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(remove_duplicates(&mut nums), 7);
        println!("{:?}", nums);
    }
}
