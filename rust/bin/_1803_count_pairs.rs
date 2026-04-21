/*
 * @Date: 2023-01-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-05
 * @FilePath: /algorithm/1803_count_pairs/count_pairs.rs
 */

fn query(num: i32, limit: i32, vec: &Vec<i32>) -> i32 {
    let mut idx = 0;
    let mut count = 0;
    for i in (0..=14).rev() {
        let bit_num = ((num >> i) & 1) as usize;
        let bit_high = (limit >> i) & 1;
        if bit_high == 1 {
            let c = vec[idx * 2 + 1 + bit_num];
            count += c;
            idx = idx * 2 + 2 - bit_num;
        } else {
            idx = idx * 2 + 1 + bit_num;
        }
    }
    return count;
}
fn insert(num: i32, vec: &mut Vec<i32>) {
    let mut idx = 0;
    for i in (0..=14).rev() {
        let bit_num = ((num >> i) & 1) as usize;
        idx = idx * 2 + 1 + bit_num;
        vec[idx] += 1;
    }
}

struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        let mut vec = vec![0_i32; 1 << 16]; //left:0,right:1
        let mut ans = 0;
        insert(nums[0], &mut vec);
        for &num in nums.iter().skip(1) {
            let a1 = query(num, high + 1, &vec);
            let a2 = query(num, low, &vec);
            ans += a1 - a2;
            insert(num, &mut vec);
        }

        return ans;
    }
}

fn main() {
    {
        let nums = vec![1, 4, 2, 7];
        let low = 2;
        let high = 6;
        let ans = 6;
        assert_eq!(Solution::count_pairs(nums, low, high), ans);
    }

    {
        let nums = vec![9, 8, 4, 2, 1];
        let low = 5;
        let high = 14;
        let ans = 8;
        assert_eq!(Solution::count_pairs(nums, low, high), ans);
    }
}
