/*
 * @Date: 2023-07-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-04
 * @FilePath: /algorithm/rust/2679_matrix_sum/matrix_sum.rs
 */

struct Solution;
impl Solution {
    pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
        for list in nums.iter_mut() {
            list.sort();
        }
        let mut res: i32 = 0;
        let (_, m) = (nums.len(), nums[0].len());
        for col in 0..m {
            res += nums.iter().map(|x| x[col]).max().unwrap();
        }
        res
    }
}

fn main() {
    let test_map = vec![
        (vec![vec![7, 2, 1], vec![6, 5, 3], vec![3, 2, 1]], 15),
        (vec![vec![1]], 1),
    ];

    for item in test_map {
        assert_eq!(Solution::matrix_sum(item.0), item.1);
    }
}
