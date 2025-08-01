struct Solution;

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let s = num.to_string();
        let t = s.clone();
        let pos = s.chars().position(|c| c != '9');
        let mut max = s.clone();
        if let Some(pos) = pos {
            let a = s.chars().nth(pos).unwrap();
            max = s.replace(a, "9");
        }
        let b = t.chars().next().unwrap();
        let min = t.replace(b, "0");
        max.parse::<i32>().unwrap() - min.parse::<i32>().unwrap()
    }
}

fn main() {
    let tests = vec![(11891, 99009), (90, 99)];

    for (num, ans) in tests {
        assert_eq!(Solution::min_max_difference(num), ans);
    }
}
