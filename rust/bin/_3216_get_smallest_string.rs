struct Solution;

impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut arr: Vec<char> = s.chars().collect();
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] && (arr[i] as u32) % 2 == (arr[i + 1] as u32) % 2 {
                arr.swap(i, i + 1);
                break;
            }
        }
        arr.iter().collect()
    }
}

fn main() {
    let tests = vec![("45320", "43520"), ("001", "001")];

    for (s, ans) in tests {
        assert_eq!(Solution::get_smallest_string(s.to_string()), ans);
    }
}
