/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-19 18:29:40
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-29 19:13:35
 */

fn premute(nums: &Vec<i32>) -> Vec<Vec<i32>> {
    // 记录结果
    let mut res: Vec<Vec<i32>> = vec![];
    // 记录追踪
    let mut trace: Vec<i32> = vec![];

    fn back_trace(nums: &Vec<i32>, trace: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if nums.len() == trace.len() {
            res.push(trace.to_vec());
            return;
        }

        for i in 0..nums.len() {
            if trace.iter().any(|&x| x == nums[i]) {
                continue;
            }
            trace.push(nums[i]);
            back_trace(nums, trace, res);
            trace.pop();
        }
    }

    back_trace(nums, &mut trace, &mut res);
    return res;
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let res = premute(&nums);
    println!("number = {}", res.len());
    for v in res {
        println!("{:?}", v);
    }
}
