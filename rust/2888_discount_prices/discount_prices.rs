struct Solution;

impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        let mut replace: Vec<String> = Vec::new();
        for word in words.iter() {
            if word.starts_with("$") && word.len() > 1 && Self::is_numeric(&word[1..]) {
                let price: f64 = word[1..].parse().unwrap();
                let discounted_price = price * (1.0 - discount as f64 / 100.0);
                replace.push(format!("${:.2}", discounted_price));
            } else {
                replace.push(word.to_string());
            }
        }
        replace.join(" ")
    }

    fn is_numeric(s: &str) -> bool {
        s.chars().all(|c| c.is_digit(10))
    }
}

fn main() {
    let tests = vec![
        (
            "there are $1 $2 and 5$ candies in the shop",
            50,
            "there are $0.50 $1.00 and 5$ candies in the shop",
        ),
        (
            "1 2 $3 4 $5 $6 7 8$ $9 $10$",
            100,
            "1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$",
        ),
    ];

    for (sentence, discount, ans) in tests {
        assert_eq!(
            Solution::discount_prices(sentence.to_string(), discount),
            ans.to_string()
        );
    }
}
