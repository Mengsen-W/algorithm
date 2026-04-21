struct Solution;

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        for i in (n + 1)..=1224444 {
            if Self::is_balance(i) {
                return i;
            }
        }
        return -1;
    }
    fn is_balance(x: i32) -> bool {
        let mut x = x;
        let mut count = [0; 10];
        while x > 0 {
            count[x as usize % 10] += 1;
            x /= 10;
        }

        for d in 0..10 {
            if count[d] > 0 && count[d] != d {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let tests = vec![(1, 22), (1000, 1333), (3000, 3133), (122645, 123233)];

    for (x, ans) in tests {
        assert_eq!(Solution::next_beautiful_number(x), ans);
    }
}
