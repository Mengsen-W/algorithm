struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();

        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as usize;
            let v = q[3] as i64;

            let mut i = l;
            while i <= r {
                nums[i] = (nums[i] * v) % Self::MOD;
                i += k;
            }
        }

        let mut res = 0;
        for x in nums {
            res ^= x;
        }
        res as i32
    }
}

fn main() {
    let tests = vec![
        (vec![1,1,1], vec![vec![0,2,1,4]], 4),
        (vec![2,3,1,5,4], vec![vec![1,4,2,3], vec![0,2,1,2]], 31),
    ];

    for (nums, queries, ans) in tests {
        assert_eq!(Solution::xor_after_queries(nums, queries), ans);
    }
}
