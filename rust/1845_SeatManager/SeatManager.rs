use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct SeatManager {
    available: BinaryHeap<Reverse<i32>>,
}

impl SeatManager {
    fn new(n: i32) -> Self {
        let mut available = BinaryHeap::new();
        for i in 1..=n {
            available.push(Reverse(i));
        }
        SeatManager { available }
    }

    fn reserve(&mut self) -> i32 {
        self.available.pop().unwrap().0
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.available.push(Reverse(seat_number));
    }
}

fn main() {
    let mut seat_manager = SeatManager::new(5); // 初始化 SeatManager ，有 5 个座位。
    seat_manager.reserve(); // 所有座位都可以预约，所以返回最小编号的座位，也就是 1 。
    seat_manager.reserve(); // 可以预约的座位为 [2,3,4,5] ，返回最小编号的座位，也就是 2 。
    seat_manager.unreserve(2); // 将座位 2 变为可以预约，现在可预约的座位为 [2,3,4,5] 。
    seat_manager.reserve(); // 可以预约的座位为 [2,3,4,5] ，返回最小编号的座位，也就是 2 。
    seat_manager.reserve(); // 可以预约的座位为 [3,4,5] ，返回最小编号的座位，也就是 3 。
    seat_manager.reserve(); // 可以预约的座位为 [4,5] ，返回最小编号的座位，也就是 4 。
    seat_manager.reserve(); // 唯一可以预约的是座位 5 ，所以返回 5 。
    seat_manager.unreserve(5); // 将座位 5 变为可以预约，现在可预约的座位为 [5] 。
}
