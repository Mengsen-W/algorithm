struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        Self::find_kth_bit_recursive(n, k)
    }

    fn find_kth_bit_recursive(n: i32, k: i32) -> char {
        if k == 1 {
            return '0';
        }
        let mid = 1 << (n - 1);
        if k == mid {
            return '1';
        } else if k < mid {
            return Self::find_kth_bit_recursive(n - 1, k);
        } else {
            let new_k = mid * 2 - k;
            return Self::invert(Self::find_kth_bit_recursive(n - 1, new_k));
        }
    }

    fn invert(bit: char) -> char {
        if bit == '0' {
            '1'
        } else {
            '0'
        }
    }
}

fn main() {
    let tests = vec![(3, 1, '0'), (4, 11, '1'), (1, 1, '0'), (2, 3, '1')];

    for (n, k, expected) in tests {
        assert_eq!(Solution::find_kth_bit(n, k), expected);
    }
}
