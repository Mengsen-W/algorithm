struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ans = vec![];
        let n = nums.len();
        for mask in 0..(1 << n) {
            let mut t = vec![];
            let mut flag = true;
            for i in 0..n {
                if (mask & (1 << i)) != 0 {
                    if i > 0 && (mask & (1 << (i - 1))) == 0 && nums[i] == nums[i - 1] {
                        flag = false;
                        break;
                    }
                    t.push(nums[i]);
                }
            }
            if flag {
                ans.push(t);
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 2, 2],
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![2, 2],
                vec![1, 2, 2],
            ],
        ),
        (vec![0], vec![vec![], vec![0]]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::subsets_with_dup(nums), ans);
    }
}
