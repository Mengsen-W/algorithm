/*
 * @Date: 2023-07-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-08
 * @FilePath: /algorithm/rust/167_two_sum/two_sum.rs
 */

struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() - 1);
        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            } else {
                break;
            }
        }
        vec![(left + 1) as i32, (right + 1) as i32]
    }
}

fn main() {
    let test_map = vec![
        (vec![2, 7, 11, 15], 9, vec![1, 2]),
        (vec![2, 3, 4], 6, vec![1, 3]),
        (vec![-1, 0], -1, vec![1, 2]),
    ];

    for (numbers, target, expect) in test_map {
        assert_eq!(Solution::two_sum(numbers, target), expect);
    }
}
