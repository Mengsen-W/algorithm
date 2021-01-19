/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-19 18:29:40
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-19 19:17:49
 */

fn premute(&nums: Vec<i32>) -> &Vec<Vec<i32>> {
    // 记录结果
    let res: Vec<Vec<i32>> = vec![];
    // 记录追踪
    let mut trace: Vec<i32> = vec![];

    fn back_trace(&nums: Vec<i32>, trace: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if nums.len() == trace.len() {
            res.push(trace);
            return;
        }

        for i in 0..nums.len() {
          if trace.iter().all(|&x|, x == nums[i]) continue;
          trace.push(nums[i]);
          backtrace(nums, trace, res);
          trace.pop();
        }
    }

    backtrace(nums, trace, res);
    return res;
}

fn main() {}
