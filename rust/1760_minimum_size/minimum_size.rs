struct Solution;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let check = |ans: i32| -> bool {
            if ans == 0 {
                return false;
            }
            let mut times = 0;
            for i in &nums {
                if i % ans == 0 {
                    times += i / ans - 1;
                } else {
                    times += i / ans;
                }

                if times > max_operations {
                    return false;
                }
            }
            times <= max_operations
        };

        let mut left = 0;
        let mut right = *nums.iter().max().unwrap();
        while left < right {
            let mid = left + ((right - left) >> 1);
            if check(mid as i32) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left as i32
    }
}

fn main() {
    let tests = vec![
        (vec![9], 2, 3),
        (vec![2, 4, 8, 2], 4, 2),
        (vec![7, 17], 2, 7),
    ];

    for (nums, max_operations, ans) in tests {
        assert_eq!(Solution::minimum_size(nums, max_operations), ans);
    }
}
