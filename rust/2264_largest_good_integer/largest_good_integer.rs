struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut res = String::new();
        let n = num.len();
        for i in 0..n - 2 {
            if num[i..i + 1] == num[i + 1..i + 2] && num[i + 1..i + 2] == num[i + 2..i + 3] {
                let current = &num[i..i + 3];
                if res.is_empty() || current > &res {
                    res = current.to_string();
                }
            }
        }
        res
    }
}

fn main() {
    let tests = vec![("6777133339", "777"), ("2300019", "000"), ("42352338", "")];

    for (num, ans) in tests {
        assert_eq!(
            Solution::largest_good_integer(num.to_string()),
            ans.to_string()
        );
    }
}
