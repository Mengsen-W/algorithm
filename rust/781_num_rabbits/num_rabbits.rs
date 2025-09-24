struct Solution;

impl Solution {
    fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for i in answers {
            *map.entry(i).or_insert(0) += 1;
        }
        let mut res: i32 = 0;
        for (x, y) in map {
            res += (x + y) / (x + 1) * (x + 1);
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![1, 1, 2], 5), (vec![10, 10, 10], 11), (vec![], 0)];

    for (answers, ans) in tests {
        assert_eq!(Solution::num_rabbits(answers), ans);
    }
}
