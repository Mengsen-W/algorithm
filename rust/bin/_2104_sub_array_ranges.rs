/*
 * @Date: 2022-03-04 00:51:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-04 02:58:29
 * @FilePath: /algorithm/2104_sub_array_ranges/sub_array_ranges.rs
 */

pub fn sub_array_ranges1(nums: Vec<i32>) -> i64 {
    nums.iter().enumerate().fold(0_i64, |mut ans, (i, num)| {
        let (mut min_val, mut max_val) = (num, num);
        nums[i..].iter().for_each(|v| {
            min_val = min_val.min(v);
            max_val = max_val.max(v);
            ans += (max_val - min_val) as i64;
        });
        ans
    })
}

pub fn sub_array_ranges2(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let (mut min_left, mut min_right, mut max_left, mut max_right) = (
        vec![0_i32; n],
        vec![0_i32; n],
        vec![0_i32; n],
        vec![0_i32; n],
    );
    let (mut min_stack, mut max_stack) = (Vec::new(), Vec::new());
    for i in 0..n {
        while !min_stack.is_empty() && nums[*min_stack.last().unwrap()] > nums[i] {
            min_stack.pop();
        }
        min_left[i] = match min_stack.last() {
            Some(v) => (*v) as i32,
            None => -1,
        };
        min_stack.push(i);

        while !max_stack.is_empty() && nums[*max_stack.last().unwrap()] <= nums[i] {
            max_stack.pop();
        }
        max_left[i] = match max_stack.last() {
            Some(v) => (*v) as i32,
            None => -1,
        };
        max_stack.push(i);
    }

    min_stack = Vec::new();
    max_stack = Vec::new();
    for i in (0..=(n - 1)).rev() {
        while !min_stack.is_empty() && nums[*min_stack.last().unwrap()] >= nums[i] {
            min_stack.pop();
        }
        min_right[i] = match min_stack.last() {
            Some(v) => (*v) as i32,
            None => n as i32,
        };
        min_stack.push(i);

        while !max_stack.is_empty() && nums[*max_stack.last().unwrap()] < nums[i] {
            max_stack.pop();
        }
        max_right[i] = match max_stack.last() {
            Some(v) => (*v) as i32,
            None => n as i32,
        };
        max_stack.push(i);
    }

    let (mut sum_max, mut sum_min) = (0_i64, 0_i64);
    for i in 0..n {
        sum_max +=
            (max_right[i] - i as i32) as i64 * (i as i32 - max_left[i]) as i64 * nums[i] as i64;
        sum_min +=
            (min_right[i] - i as i32) as i64 * (i as i32 - min_left[i]) as i64 * nums[i] as i64;
    }
    sum_max - sum_min
}

fn main() {
    assert_eq!(sub_array_ranges1(vec![1, 2, 3]), 4);
    assert_eq!(sub_array_ranges2(vec![1, 2, 3]), 4);

    assert_eq!(sub_array_ranges1(vec![1, 3, 3]), 4);
    assert_eq!(sub_array_ranges2(vec![1, 3, 3]), 4);

    assert_eq!(sub_array_ranges1(vec![4, -2, -3, 4, 1]), 59);
    assert_eq!(sub_array_ranges2(vec![4, -2, -3, 4, 1]), 59);

}
