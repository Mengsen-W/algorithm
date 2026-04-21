struct Solution;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = n + 1;

        if n < 3 {
            return -1;
        }

        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                if nums[i] != nums[j] {
                    continue;
                }
                for k in j + 1..n {
                    if nums[j] == nums[k] {
                        ans = ans.min(k - i);
                        break;
                    }
                }
            }
        }

        if ans == n + 1 {
            -1
        } else {
            (ans * 2) as i32
        }
    }
}

fn main() {
    let tests = vec![
        ( vec![ 1, 2, 1, 1, 3 ], 6 ),
        ( vec![ 1, 1, 2, 3, 2, 1, 2 ], 8 ),
        ( vec![ 1 ], -1 ),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::minimum_distance(nums), ans);
    }
}
