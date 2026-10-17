struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut digit_sum = vec![0; n];
        for i in 0..n {
            let mut x = nums[i];
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            digit_sum[i] = sum;
        }

        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by_key(|&i| (digit_sum[i], nums[i]));

        let mut target_pos = vec![0; n];
        for (pos, &idx) in indices.iter().enumerate() {
            target_pos[idx] = pos;
        }

        let mut parent: Vec<usize> = (0..n).collect();
        let mut components = n;

        for i in 0..n {
            let ri = Self::find(&mut parent, i);
            let rj = Self::find(&mut parent, target_pos[i]);
            if ri != rj {
                parent[ri] = rj;
                components -= 1;
            }
        }

        (n - components) as i32
    }

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]);
        }
        parent[x]
    }
}

fn main() {
    let tests = vec![
        (vec![37, 100], 1),
        (vec![22, 14, 33, 7], 0),
        (vec![18, 43, 34, 16], 2),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::min_swaps(nums), expected);
    }
}
