struct Solution;

impl Solution {
    pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        // 第一次遍历编码最终值
        for i in 0..n {
            nums[i] += 1000 * (nums[nums[i] as usize] % 1000);
        }
        // 第二次遍历修改为最终值
        for i in 0..n {
            nums[i] /= 1000;
        }
        nums
    }
}

fn main() {
    let tests = vec![
        (vec![0, 2, 1, 5, 3, 4], vec![0, 1, 2, 4, 5, 3]),
        (vec![5, 0, 1, 2, 3, 4], vec![4, 5, 0, 1, 2, 3]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::build_array(nums), ans);
    }
}
