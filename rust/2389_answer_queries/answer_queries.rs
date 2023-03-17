/*
 * @Date: 2023-03-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-17
 * @FilePath: /algorithm/rust/2389_answer_queries/answer_queries.rs
 */

pub fn answer_queries(mut nums: Vec<i32>, mut queries: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    nums.iter_mut().fold(0, |s, x| {
        *x += s;
        *x
    });
    queries
        .iter_mut()
        .for_each(|x| *x = nums.partition_point(|y| y <= x) as i32);
    queries
}

fn main() {
    {
        let nums = vec![4, 5, 2, 1];
        let queries = vec![3, 10, 21];
        let ans = vec![2, 3, 4];
        assert_eq!(answer_queries(nums, queries), ans);
    }

    {
        let nums = vec![2, 3, 4, 5];
        let queries = vec![1];
        let ans = vec![0];
        assert_eq!(answer_queries(nums, queries), ans);
    }
}
