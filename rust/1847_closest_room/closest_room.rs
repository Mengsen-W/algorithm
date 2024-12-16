struct Solution;

use std::collections::BTreeSet;

#[derive(Debug)]
struct Event {
    // 事件的类型，0 表示房间，1 表示询问
    event_type: i32,
    // 房间的 size 或者询问的 minSize
    size: i32,
    // 房间的 roomId 或者询问的 preferred
    id: i32,
    // 房间在数组 room 中的原始编号或者询问在数组 queries 中的原始编号
    origin: usize,
}

impl Event {
    fn new(event_type: i32, size: i32, id: i32, origin: usize) -> Self {
        Event {
            event_type,
            size,
            id,
            origin,
        }
    }
}

// 自定义比较函数，按照事件的 size 降序排序
// 如果 size 相同，优先考虑房间
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.event_type == other.event_type
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .size
            .cmp(&self.size)
            .then(self.event_type.cmp(&other.event_type))
    }
}

impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut events = Vec::new();
        for (i, room) in rooms.iter().enumerate() {
            // 房间事件
            events.push(Event::new(0, room[1], room[0], i));
        }
        for (i, query) in queries.iter().enumerate() {
            // 询问事件
            events.push(Event::new(1, query[1], query[0], i));
        }

        // 对事件进行排序
        events.sort();
        // 用于存储房间的 roomId 的有序集合
        let mut valid_rooms = BTreeSet::new();
        let mut ans = vec![-1; queries.len()];
        for event in events {
            if event.event_type == 0 {
                // 房间事件，将 roomId 加入有序集合
                valid_rooms.insert(event.id);
            } else {
                // 询问事件
                let mut dist = i32::MAX;
                // 查找大于等于 preferred 的最小房间
                if let Some(&ceiling) = valid_rooms.range(event.id..).next() {
                    if ceiling - event.id < dist {
                        dist = ceiling - event.id;
                        ans[event.origin] = ceiling;
                    }
                }

                // 查找小于 preferred 的最大房间
                if let Some(&floor) = valid_rooms.range(..event.id).next_back() {
                    if event.id - floor <= dist {
                        ans[event.origin] = floor;
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![2, 2], vec![1, 2], vec![3, 2]],
            vec![vec![3, 1], vec![3, 3], vec![5, 2]],
            vec![3, -1, 3],
        ),
        (
            vec![vec![1, 4], vec![2, 3], vec![3, 5], vec![4, 1], vec![5, 2]],
            vec![vec![2, 3], vec![2, 4], vec![2, 5]],
            vec![2, 1, 3],
        ),
    ];

    for (rooms, queries, ans) in tests {
        assert_eq!(Solution::closest_room(rooms, queries), ans);
    }
}
