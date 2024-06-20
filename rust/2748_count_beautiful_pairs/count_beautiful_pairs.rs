struct Solution;
impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len() as i32;
        for i in 0..n {
            let mut x = nums[i as usize];
            while x >= 10 {
                x /= 10;
            }
            for j in (i + 1)..n {
                if Self::gcd(x, nums[j as usize] % 10) == 1 {
                    res += 1;
                }
            }
        }
        res
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

fn main() {
    let tests = vec![(vec![2, 5, 1, 4], 5), (vec![11, 21, 12], 2)];

    for (nums, ans) in tests {
        assert_eq!(Solution::count_beautiful_pairs(nums), ans);
    }
}
