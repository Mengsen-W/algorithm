struct Allocator {
    n: usize,
    memory: Vec<i32>,
}

impl Allocator {
    fn new(n: i32) -> Self {
        Allocator {
            n: n as usize,
            memory: vec![0; n as usize],
        }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut count = 0;
        for i in 0..self.n {
            if self.memory[i] != 0 {
                count = 0;
            } else {
                count += 1;
                if count == size {
                    for j in (i as i32 - count + 1)..=i as i32 {
                        self.memory[j as usize] = m_id;
                    }
                    return i as i32 - count + 1;
                }
            }
        }
        -1
    }

    fn free_memory(&mut self, m_id: i32) -> i32 {
        let mut count = 0;
        for i in 0..self.n {
            if self.memory[i] == m_id {
                count += 1;
                self.memory[i] = 0;
            }
        }
        count
    }
}

fn main() {
    let mut loc = Allocator::new(10);
    assert_eq!(loc.allocate(1, 1), 0);
    assert_eq!(loc.allocate(1, 2), 1);
    assert_eq!(loc.allocate(1, 3), 2);
    assert_eq!(loc.free_memory(2), 1);
    assert_eq!(loc.allocate(3, 4), 3);
    assert_eq!(loc.allocate(1, 1), 1);
    assert_eq!(loc.allocate(1, 1), 6);
    assert_eq!(loc.free_memory(1), 3);
    assert_eq!(loc.allocate(10, 2), -1);
    assert_eq!(loc.free_memory(7), 0);
}
