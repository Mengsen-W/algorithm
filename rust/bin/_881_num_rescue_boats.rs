struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut ans = 0;
        let (mut i, mut j) = (0, people.len() - 1);
        while i < j {
            if people[i] + people[j] <= limit {
                i += 1;
            }
            j -= 1;
            ans += 1;
        }
        ans + if i == j { 1 } else { 0 }
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2], 3, 1),
        (vec![3, 2, 2, 1], 3, 3),
        (vec![3, 5, 3, 4], 5, 4),
    ];

    for (people, limit, ans) in tests {
        assert_eq!(Solution::num_rescue_boats(people, limit), ans);
    }
}
