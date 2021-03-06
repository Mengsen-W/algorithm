/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-06 10:03:52
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-06 10:10:30
 */

fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let nums = nums.repeat(2);
    let mut answer = vec![0; n];
    for i in 0..n {
        let mut j = i + 1;
        while j < n * 2 && nums[i] >= nums[j] {
            j += 1;
        }
        if j == n * 2 {
            answer[i] = -1;
        } else {
            answer[i] = nums[j];
        }
    }
    answer
}

fn main() {
    assert_eq!(next_greater_elements(vec![1, 2, 1]), vec![2, -1, 2]);
}
