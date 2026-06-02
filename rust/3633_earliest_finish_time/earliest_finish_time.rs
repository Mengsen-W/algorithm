struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let n = land_start_time.len();
        let m = water_start_time.len();
        let mut res = i32::MAX;
        for i in 0..n {
            for j in 0..m {
                let land = land_start_time[i] + land_duration[i];
                let land_water = land.max(water_start_time[j]) + water_duration[j];
                res = res.min(land_water);

                let water = water_start_time[j] + water_duration[j];
                let water_land = water.max(land_start_time[i]) + land_duration[i];
                res = res.min(water_land);
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![2, 8], vec![4, 1], vec![6], vec![3], 9),
        (vec![5], vec![3], vec![1], vec![10], 14),
    ];

    for (land_start_time, land_duration, water_start_time, water_duration, expected) in tests {
        assert_eq!(
            Solution::earliest_finish_time(
                land_start_time,
                land_duration,
                water_start_time,
                water_duration
            ),
            expected
        );
    }
}
