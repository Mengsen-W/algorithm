struct Solution;

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        const P: i32 = 1_000_000_007;
        const MAX_N: usize = 100_005;
        let mut f = vec![0; MAX_N];
        f[0] = 1;
        for i in 1..MAX_N {
            f[i] = (f[i - 1] << 1) % P;
        }
        let mut nums = nums;
        nums.sort();

        let mut ans = 0;
        for i in 0..nums.len() {
            if nums[i] * 2 > target {
                break;
            }
            let max_value = target - nums[i];
            let pos = Self::binary_search(&nums, max_value) - 1;
            let contribute = if pos >= i { f[pos - i] } else { 0 };
            ans = (ans + contribute) % P;
        }

        ans
    }

    fn binary_search(nums: &Vec<i32>, target: i32) -> usize {
        let mut low = 0;
        let mut high = nums.len();
        while low < high {
            let mid = (high - low) / 2 + low;
            if mid == nums.len() {
                return mid;
            }
            let num = nums[mid];
            if num <= target {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }
}

fn main() {
    let tests = vec![
        (vec![3, 5, 6, 7], 9, 4),
        (vec![3, 3, 6, 8], 10, 6),
        (vec![2, 3, 3, 4, 6, 7], 12, 61),
    ];

    for (nums, target, expected) in tests {
        assert_eq!(Solution::num_subseq(nums, target), expected);
    }
}
