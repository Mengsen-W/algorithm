struct Solution;

impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let indices: Vec<usize> = nums
            .iter()
            .enumerate()
            .filter_map(|(i, &num)| if num == x { Some(i) } else { None })
            .collect();

        queries
            .iter()
            .map(|&q| {
                if (q as usize) > indices.len() {
                    -1
                } else {
                    indices[(q - 1) as usize] as i32
                }
            })
            .collect()
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1, vec![0, -1, 2, -1]),
        (vec![1, 2, 3], vec![10], 5, vec![-1]),
    ];

    for (nums, queries, x, ans) in tests {
        assert_eq!(Solution::occurrences_of_element(nums, queries, x), ans);
    }
}
