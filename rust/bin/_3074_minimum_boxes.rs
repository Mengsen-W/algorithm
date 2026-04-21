struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut sum: i32 = apple.iter().sum();
        let mut sorted_capacity = capacity.clone();
        sorted_capacity.sort_by(|a, b| b.cmp(a));

        let mut need = 0;
        while sum > 0 {
            sum -= sorted_capacity[need as usize];
            need += 1;
        }

        need
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 2], vec![4, 3, 1, 5, 2], 2),
        (vec![5, 5, 5], vec![2, 4, 2, 7], 4),
    ];

    for (apple, capacity, expected) in tests {
        assert_eq!(Solution::minimum_boxes(apple, capacity), expected);
    }
}
