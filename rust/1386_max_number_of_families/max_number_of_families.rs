struct Solution;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let left = 0b11110000;
        let middle = 0b11000011;
        let right = 0b00001111;

        let mut occupied: HashMap<i32, i32> = HashMap::new();
        for seat in &reserved_seats {
            if seat[1] >= 2 && seat[1] <= 9 {
                let row = seat[0];
                let entry = occupied.entry(row).or_insert(0);
                *entry |= 1 << (seat[1] - 2);
            }
        }

        let mut ans = (n - occupied.len() as i32) * 2;
        for &bitmask in occupied.values() {
            if (bitmask | left) == left
                || (bitmask | middle) == middle
                || (bitmask | right) == right
            {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            3,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 8],
                vec![2, 6],
                vec![3, 1],
                vec![3, 10],
            ],
            4,
        ),
        (2, vec![vec![2, 1], vec![1, 8], vec![2, 6]], 2),
        (4, vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]], 4),
    ];

    for (n, reserved_seats, expected) in tests {
        assert_eq!(
            Solution::max_number_of_families(n, reserved_seats),
            expected
        );
    }
}
