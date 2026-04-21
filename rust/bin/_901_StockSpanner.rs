struct StockSpanner {
    mono_stack: Vec<Vec<i32>>,
}

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl StockSpanner {
    fn new() -> Self {
        Self {
            mono_stack: vec![], // [[price, days]]
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut days = 1;
        while !self.mono_stack.is_empty() && price >= self.mono_stack.last().unwrap()[0] {
            days += self.mono_stack.pop().unwrap()[1];
        }
        self.mono_stack.push(vec![price, days]);
        days
    }
}

fn main() {
    let mut s = StockSpanner::new();

    assert_eq!(s.next(100), 1);
    assert_eq!(s.next(80), 1);
    assert_eq!(s.next(60), 1);
    assert_eq!(s.next(70), 2);
    assert_eq!(s.next(60), 1);
    assert_eq!(s.next(75), 4);
    assert_eq!(s.next(85), 6);
}
