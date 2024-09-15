struct Solution;

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut c = 0;
        for interval in &nums {
            c = c.max(interval[1]);
        }

        let mut count = vec![0; (c + 1) as usize];
        for interval in &nums {
            for i in interval[0]..=interval[1] {
                count[i as usize] += 1;
            }
        }

        let mut ans = 0;
        for i in 1..=c {
            if count[i as usize] > 0 {
                ans += 1;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![3, 6], vec![1, 5], vec![4, 7]], 7),
        (vec![vec![1, 3], vec![5, 8]], 7),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::number_of_points(nums), ans);
    }
}
