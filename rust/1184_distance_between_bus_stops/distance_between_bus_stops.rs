/*
 * @Date: 2022-07-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-24
 * @FilePath: /algorithm/1184_distance_between_bus_stops/distance_between_bus_stops.rs
 */

pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    if start > destination {
        return distance_between_bus_stops(distance, destination, start);
    }
    let (start, end) = (start as usize, destination as usize);
    (&distance[start..end]).iter().sum::<i32>().min(
        (&distance[0..start]).iter().sum::<i32>()
            + (&distance[end..distance.len()]).iter().sum::<i32>(),
    )
}

fn main() {
    {
        let distance = vec![1, 2, 3, 4];
        let start = 0;
        let destination = 1;
        assert_eq!(distance_between_bus_stops(distance, start, destination), 1);
    }

    {
        let distance = vec![1, 2, 3, 4];
        let start = 0;
        let destination = 2;
        assert_eq!(distance_between_bus_stops(distance, start, destination), 3);
    }

    {
        let distance = vec![1, 2, 3, 4];
        let start = 0;
        let destination = 3;
        assert_eq!(distance_between_bus_stops(distance, start, destination), 4);
    }
}
