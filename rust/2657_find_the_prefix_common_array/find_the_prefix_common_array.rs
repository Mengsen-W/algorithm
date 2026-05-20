struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let (mut p, mut q) = (0u64, 0u64);

        for i in 0..a.len() {
            p |= 1u64 << (a[i] as u64);
            q |= 1u64 << (b[i] as u64);
            a[i] = (p & q).count_ones() as i32;
        }

        a
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 2, 4], vec![3, 1, 2, 4], vec![0, 2, 3, 4]),
        (vec![2, 3, 1], vec![3, 1, 2], vec![0, 1, 3]),
    ];

    for (a, b, expected) in tests {
        assert_eq!(Solution::find_the_prefix_common_array(a, b), expected);
    }
}
