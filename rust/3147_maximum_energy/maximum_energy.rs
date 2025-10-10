struct Solution;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let n = energy.len();
        let k_usize = k as usize;
        let mut ans = i32::MIN;

        for i in (n - k_usize)..n {
            let mut sum = 0;
            let mut j = i;
            while j < n {
                sum += energy[j];
                if sum > ans {
                    ans = sum;
                }
                if j < k_usize {
                    break;
                }
                j -= k_usize;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![5, 2, -10, -5, 1], 3, 3), (vec![-2, -3, -1], 2, -1)];

    for (energy, k, ans) in tests {
        assert_eq!(Solution::maximum_energy(energy, k), ans);
    }
}
