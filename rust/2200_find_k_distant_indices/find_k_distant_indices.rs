struct Solution;

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut r = 0; // 未被判断过的最小下标
        let n = nums.len();
        for j in 0..n {
            if nums[j] == key {
                let l = r.max(j as i32 - k);
                r = (n as i32 - 1).min(j as i32 + k) + 1;
                for i in l..r {
                    res.push(i);
                }
            }
        }
        res
    }
}

fn main() {
  let tests = vec![
      (vec![ 3, 4, 9, 1, 3, 9, 5 ], 9, 1, vec![ 1, 2, 3, 4, 5, 6 ]),
      (vec![ 2, 2, 2, 2, 2 ], 2, 2, vec![ 0, 1, 2, 3, 4 ]),
  ];

  for (t, key, k, expected) in tests {
      assert_eq!(Solution::find_k_distant_indices(t, key, k), expected);
  }
}