struct Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashSet;
        let mut cities: HashSet<String> = HashSet::new();
        for path in &paths {
            cities.insert(path[0].clone());
        }
        for path in &paths {
            if !cities.contains(&path[1]) {
                return path[1].clone();
            }
        }
        String::new()
    }
}

fn main() {
    let tests = vec![
        (
            [
                ["London", "New York"],
                ["New York", "Lima"],
                ["Lima", "Sao Paulo"],
            ],
            "Sao Paulo",
        ),
        ([["B", "C"], ["D", "B"], ["C", "A"]], "A"),
    ];

    for (paths, ans) in tests {
        assert_eq!(
            Solution::dest_city(
                paths
                    .iter()
                    .map(|path| path.iter().map(|s| s.to_string()).collect())
                    .collect()
            ),
            ans.to_string()
        );
    }
}
