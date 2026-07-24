struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let max_val = nums.iter().max().copied().unwrap_or(0) as usize;
        let mut u = 1;
        while u <= max_val {
            u <<= 1;
        }
        let mut one = vec![false; u];
        let mut two = vec![false; u];
        let mut three = vec![false; u];
        for &v in &nums {
            one[v as usize] = true;
            for x in 0..u {
                if one[x] {
                    two[x ^ v as usize] = true;
                }
            }
        }
        for &v in &nums {
            for x in 0..u {
                if two[x] {
                    three[x ^ v as usize] = true;
                }
            }
        }
        three.iter().filter(|&&b| b).count() as i32
    }
}

fn main() {
    let tests = vec![(vec![1, 3], 2), (vec![6, 7, 8, 9], 4)];

    for (nums, ans) in tests {
        assert_eq!(Solution::unique_xor_triplets(nums), ans);
    }
}
