struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        s.chars()
            .fold([0, 0], |mut array, c| {
                if c == '(' {
                    array[0] += 1;
                } else if array[0] > 0 {
                    array[0] -= 1;
                } else {
                    array[1] += 1;
                }

                array
            })
            .iter()
            .sum()
    }
}

fn main() {
    let tests = vec![("())", 1), ("(((", 3)];

    for (s, expected) in tests {
        assert_eq!(Solution::min_add_to_make_valid(s.to_string()), expected);
    }
}
