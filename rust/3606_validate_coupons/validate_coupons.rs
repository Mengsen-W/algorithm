struct Solution;

impl Solution {
    pub fn check(code: &str, is_active: bool) -> bool {
        for c in code.chars() {
            if c != '_' && !c.is_ascii_alphanumeric() {
                return false;
            }
        }
        is_active
    }

    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let mut groups: Vec<Vec<String>> = vec![vec![], vec![], vec![], vec![]];
        let mut ans: Vec<String> = Vec::new();

        for i in 0..code.len() {
            if !code[i].is_empty() && Self::check(&code[i], is_active[i]) {
                match business_line[i].as_str() {
                    "electronics" => groups[0].push(code[i].clone()),
                    "grocery" => groups[1].push(code[i].clone()),
                    "pharmacy" => groups[2].push(code[i].clone()),
                    "restaurant" => groups[3].push(code[i].clone()),
                    _ => {}
                }
            }
        }

        for group in groups.iter_mut() {
            group.sort();
            ans.extend(group.clone());
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec!["SAVE20", "", "PHARMA5", "SAVE@20"],
            vec!["restaurant", "grocery", "pharmacy", "restaurant"],
            vec![true, true, true, true],
            vec!["PHARMA5", "SAVE20"],
        ),
        (
            vec!["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"],
            vec!["grocery", "electronics", "invalid"],
            vec![false, true, true],
            vec!["ELECTRONICS_50"],
        ),
    ];

    for (code, business_line, is_active, expected) in tests {
        assert_eq!(
            Solution::validate_coupons(
                code.iter().map(|s| s.to_string()).collect(),
                business_line.iter().map(|s| s.to_string()).collect(),
                is_active
            ),
            expected
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }
}
