use std::cmp::max;

struct BrowserHistory {
    urls: Vec<String>,
    curr_index: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            urls: vec![homepage],
            curr_index: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.urls.truncate(self.curr_index + 1);
        self.urls.push(url);
        self.curr_index += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        self.curr_index = max(self.curr_index as i32 - steps, 0) as usize;
        return self.urls[self.curr_index].clone();
    }

    fn forward(&mut self, steps: i32) -> String {
        self.curr_index = std::cmp::min(self.curr_index + steps as usize, self.urls.len() - 1);
        return self.urls[self.curr_index].clone();
    }
}

fn main() {
    let mut browser_history = BrowserHistory::new("leetcode.com".to_string());
    browser_history.visit("google.com".to_string());
    browser_history.visit("facebook.com".to_string());
    browser_history.visit("youtube.com".to_string());
    assert_eq!(browser_history.back(1), "facebook.com".to_string());
    assert_eq!(browser_history.back(1), "google.com".to_string());
    assert_eq!(browser_history.forward(1), "facebook.com".to_string());
    browser_history.visit("linkedin.com".to_string());
    assert_eq!(browser_history.forward(2), "linkedin.com".to_string());
    assert_eq!(browser_history.back(2), "google.com".to_string());
    assert_eq!(browser_history.back(7), "leetcode.com".to_string());
}
