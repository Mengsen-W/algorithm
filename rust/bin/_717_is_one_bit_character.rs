struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let n = bits.len() as i32;
        let mut i = n - 2;
        while i >= 0 && bits[i as usize] == 1 {
            i -= 1;
        }
        (n - i) % 2 == 0
    }
}

fn main() {
    let tests = vec![(vec![1, 0, 0], true), (vec![1, 1, 1, 0], false)];

    for (bits, expected) in tests {
        assert_eq!(Solution::is_one_bit_character(bits), expected);
    }
}
