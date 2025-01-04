use std::cell::RefCell;
use std::rc::Rc;

const MAX_TIME: i32 = 1_0000_0000_0;

/// [start, end)表示当前节点对应的区间
/// max_count: 表示当前节点对应的区间[start, end)中某个时间点被预定的最大次数
/// lazy_flag：懒标记，表示当前节点已经更新，但是其子节点还未更新
/// 1：表示当前节点被预定了1次但是其子节点还没更新，2：表示当前节点被预定了2次但是其子节点还没更新，...
struct SegmentTree {
    start: i32,
    end: i32,
    max_count: i32,
    lazy_flag: i32,
    left_node: Option<Rc<RefCell<SegmentTree>>>,
    right_node: Option<Rc<RefCell<SegmentTree>>>,
}

impl SegmentTree {
    fn new(start: i32, end: i32) -> Self {
        Self {
            start,
            end,
            max_count: 0,
            lazy_flag: 0,
            left_node: None,
            right_node: None,
        }
    }

    fn update_cur_node(&mut self, count: i32) {
        self.max_count += count;
        self.lazy_flag += count;
    }

    /// 预定时间段 [left, right)
    fn update(&mut self, left: i32, right: i32) {
        if left >= self.end || right <= self.start {
            return;
        }
        if left <= self.start && right >= self.end {
            self.update_cur_node(1);
            return;
        }

        self.update_cross_range(left, right);
    }

    fn update_cross_range(&mut self, left: i32, right: i32) {
        let mid = self.start + (self.end - self.start) / 2;
        let left_node = self
            .left_node
            .get_or_insert(Rc::new(RefCell::new(SegmentTree::new(self.start, mid))));
        let right_node = self
            .right_node
            .get_or_insert(Rc::new(RefCell::new(SegmentTree::new(mid, self.end))));

        if self.lazy_flag > 0 {
            left_node.borrow_mut().update_cur_node(self.lazy_flag);
            right_node.borrow_mut().update_cur_node(self.lazy_flag);
            self.lazy_flag = 0;
        }

        left_node.borrow_mut().update(left, right);
        right_node.borrow_mut().update(left, right);

        self.max_count = i32::max(left_node.borrow().max_count, right_node.borrow().max_count);
    }

    /// 查询时间段 [left, right)中，被预定的最大次数
    fn query(&mut self, left: i32, right: i32) -> i32 {
        if left >= self.end || right <= self.start {
            return 0;
        }
        if left <= self.start && right >= self.end {
            return self.max_count;
        }

        self.query_cross_range(left, right)
    }

    fn query_cross_range(&mut self, left: i32, right: i32) -> i32 {
        let mid = self.start + (self.end - self.start) / 2;
        let left_node = self
            .left_node
            .get_or_insert(Rc::new(RefCell::new(SegmentTree::new(self.start, mid))));
        let right_node = self
            .right_node
            .get_or_insert(Rc::new(RefCell::new(SegmentTree::new(mid, self.end))));

        if self.lazy_flag > 0 {
            left_node.borrow_mut().update_cur_node(self.lazy_flag);
            right_node.borrow_mut().update_cur_node(self.lazy_flag);
            self.lazy_flag = 0;
        }

        i32::max(
            left_node.borrow_mut().query(left, right),
            right_node.borrow_mut().query(left, right),
        )
    }
}

struct MyCalendarThree {
    max_count: i32,
    root: SegmentTree,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            max_count: 0,
            root: SegmentTree::new(0, MAX_TIME),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        self.root.update(start_time, end_time);
        let cur_max_count = self.root.query(start_time, end_time);
        self.max_count = i32::max(self.max_count, cur_max_count);
        self.max_count
    }
}

fn main() {
    let mut my_calendar_three = MyCalendarThree::new();
    assert_eq!(my_calendar_three.book(10, 20), 1);
    assert_eq!(my_calendar_three.book(50, 60), 1);
    assert_eq!(my_calendar_three.book(10, 40), 2);
    assert_eq!(my_calendar_three.book(5, 15), 3);
    assert_eq!(my_calendar_three.book(5, 10), 3);
    assert_eq!(my_calendar_three.book(25, 55), 3);
}
