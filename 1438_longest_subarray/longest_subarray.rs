/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-21 09:12:53
 * @Last Modified by:   Mengsen.Wang
 * @Last Modified time: 2021-02-21 09:12:53
 */

use std::collections::BinaryHeap;

fn longest_sub_array(nums: Vec<i32>, limit: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut max_heap = BinaryHeap::new();
    let mut min_heap = BinaryHeap::new();
    let mut max_val;
    let mut min_val;
    let mut max_index;
    let mut min_index;
    let mut res = 0;

    while right < nums.len() {
        max_heap.push((nums[right], right));
        min_heap.push((-nums[right], right));

        max_val = max_heap.peek().unwrap().0;
        min_val = -min_heap.peek().unwrap().0;
        //println!("max_heap: {:#?}", max_heap);
        //println!("min_heap: {:#?}", min_heap);

        while (max_val - min_val).abs() > limit {
            left += 1;
            min_index = min_heap.peek().unwrap().1;
            max_index = max_heap.peek().unwrap().1;

            while min_index < left {
                min_heap.pop();
                min_index = min_heap.peek().unwrap().1;
            }
            while max_index < left {
                max_heap.pop();
                max_index = max_heap.peek().unwrap().1;
            }

            max_val = max_heap.peek().unwrap().0;
            min_val = -min_heap.peek().unwrap().0;
        }
        res = res.max((right - left) as i32 + 1);

        right += 1;
    }

    res
}

fn main() {
    let mut nums: Vec<i32>;
    nums = vec![8, 2, 4, 7];
    assert_eq!(longest_sub_array(nums, 4), 2);
    nums = vec![10, 1, 2, 4, 7, 2];
    assert_eq!(longest_sub_array(nums, 5), 4);
    nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
    assert_eq!(longest_sub_array(nums, 0), 3);
}
