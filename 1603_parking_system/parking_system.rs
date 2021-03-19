/*
 * @Date: 2021-03-19 08:21:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-19 08:26:19
 */

struct ParkingSystem([i16; 3]);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    #[inline(always)]
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self([big as i16, medium as i16, small as i16])
    }
    #[inline(always)]
    fn add_car(&mut self, car_type: i32) -> bool {
        if self.0[car_type as usize - 1] == 0 {
            return false;
        } else {
            self.0[car_type as usize - 1] -= 1;
        }
        true
    }
}

fn main() {
    let mut obj = ParkingSystem::new(1, 1, 0);
    assert!(obj.add_car(1));
    assert!(obj.add_car(2));
    assert!(!obj.add_car(3));
    assert!(!obj.add_car(1));
}
