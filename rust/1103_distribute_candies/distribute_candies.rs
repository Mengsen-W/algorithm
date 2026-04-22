struct Solution;

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let n = num_people;
        // how many people received complete gifts
        let p = (f64::sqrt(2.0 * candies as f64 + 0.25) - 0.5) as i32;
        let remaining = candies - (p + 1) * p / 2;
        let rows = p / n;
        let cols = p % n;
        let mut d = vec![0; n as usize];

        for i in 0..n {
            // Complete rows
            d[i as usize] = (i + 1) * rows + ((rows * (rows - 1)) / 2) * n;
            // Columns in the last row
            if i < cols {
                d[i as usize] += i + 1 + rows * n;
            }
        }
        // Remaining candies
        d[cols as usize] += remaining;
        d
    }
}

fn main() {
    let tests = vec![(7, 4, vec![1, 2, 3, 1]), (10, 3, vec![5, 2, 3])];

    for (candies, num_people, ans) in tests {
        assert_eq!(Solution::distribute_candies(candies, num_people), ans);
    }
}
