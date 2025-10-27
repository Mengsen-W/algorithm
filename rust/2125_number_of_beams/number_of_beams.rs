struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut last = 0;
        let mut ans = 0;
        for line in bank {
            let cnt = line.chars().filter(|&c| c == '1').count() as i32;
            if cnt != 0 {
                ans += last * cnt;
                last = cnt;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec!["011001", "000000", "010100", "001000"], 8),
        (vec!["000", "111", "000"], 0),
    ];

    for (bank, ans) in tests {
        assert_eq!(
            Solution::number_of_beams(bank.iter().map(|s| s.to_string()).collect()),
            ans
        );
    }
}
