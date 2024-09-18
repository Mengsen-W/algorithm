struct Solution;

impl Solution {
    pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        let mut buses = buses;
        let mut passengers = passengers;
        buses.sort();
        passengers.sort();
        let mut pos = 0;
        let mut space = 0;

        for &arrive in &buses {
            space = capacity;
            while space > 0 && pos < passengers.len() && passengers[pos] <= arrive {
                space -= 1;
                pos += 1;
            }
        }

        pos -= 1;
        let mut last_catch_time = if space > 0 {
            *buses.last().unwrap()
        } else {
            passengers[pos]
        };
        while pos >= 0 && passengers[pos as usize] == last_catch_time {
            pos -= 1;
            last_catch_time -= 1;
        }

        last_catch_time
    }
}

fn main() {
    let tests = vec![
        (vec![10, 20], vec![2, 17, 18, 19], 2, 16),
        (vec![20, 30, 10], vec![19, 13, 26, 4, 25, 11, 21], 2, 20),
    ];

    for (buses, passengers, capacity, ans) in tests {
        assert_eq!(
            Solution::latest_time_catch_the_bus(buses, passengers, capacity),
            ans
        );
    }
}
