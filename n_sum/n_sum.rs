/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-09 17:29:54
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-09 17:43:53
 */

fn n_sum(nums: &Vec<i32>, n: usize, start: usize, target: i32) -> Vec<Vec<i32>> {
    let size = nums.len();
    let mut res: Vec<Vec<i32>> = vec![vec![]; 0];
    if n < 2 || size < n {
        return res;
    }

    if n == 2 {
        let mut low = start as usize;
        let mut high = size - 1;
        while low < high {
            let sum = nums[low] + nums[high];
            let left = nums[low];
            let right = nums[high];
            if sum < target {
                while low < high && nums[low] == left {
                    low += 1;
                }
            } else if sum > target {
                while low < high && nums[high] == right {
                    high -= 1;
                }
            } else {
                res.push(vec![left, right]);
                while low < high && nums[low] == left {
                    low += 1;
                }
                while low < high && nums[high] == right {
                    high -= 1;
                }
            }
        }
    } else {
        for mut i in start..size {
            let sub: Vec<Vec<i32>> = n_sum(nums, n - 1, i + 1, target - nums[i]);
            for mut arr in sub {
                arr.push(nums[i]);
                res.push(arr);
            }
            while i < size - 1 && nums[i] == nums[i + 1] {
                i += 1;
            }
        }
    }
    res
}

fn main() {
    let nums = vec![1, 2, 2, 3, 4, 6];
    let res = n_sum(&nums, 4, 0, 14);
    println!("{:#?}", res);
}
