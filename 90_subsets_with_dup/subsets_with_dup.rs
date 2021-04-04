/*
 * @Date: 2021-04-04 18:39:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-04 18:50:03
 */

fn dfs(ret: &mut Vec<Vec<i32>>, nums: &Vec<i32>, cur: &mut Vec<i32>, start: usize) {
    ret.push(cur.clone());
    for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] {
            continue;
        }
        cur.push(nums[i]);
        dfs(ret, nums, cur, i + 1);
        cur.pop();
    }
}

fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut cur = vec![];
    let mut ret = vec![];
    dfs(&mut ret, &nums, &mut cur, 0);
    ret
}

fn main() {
    {
        let nums = vec![1, 2, 2];
        println!("{:?}", subsets_with_dup(nums));
    }
    {
        let nums = vec![0];
        println!("{:?}", subsets_with_dup(nums));
    }
}
