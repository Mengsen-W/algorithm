pub struct Spreadsheet {
    grid: Vec<Vec<i32>>,
}

impl Spreadsheet {
    pub fn new(rows: i32) -> Self {
        Spreadsheet {
            grid: vec![vec![0; 27]; (rows + 1) as usize],
        }
    }

    pub fn set_cell(&mut self, cell: String, value: i32) {
        let (x, y) = self.get_pos(&cell);
        self.grid[x][y] = value;
    }

    pub fn reset_cell(&mut self, cell: String) {
        let (x, y) = self.get_pos(&cell);
        self.grid[x][y] = 0;
    }

    pub fn get_value(&self, formula: String) -> i32 {
        let i = formula.find('+').unwrap();
        let cell1 = &formula[1..i];
        let cell2 = &formula[i + 1..];
        self.get_cell_val(cell1) + self.get_cell_val(cell2)
    }

    fn get_pos(&self, cell: &str) -> (usize, usize) {
        let x = cell[1..].parse::<usize>().unwrap();
        let y = cell.chars().next().unwrap() as usize - 'A' as usize;
        (x, y)
    }

    fn get_cell_val(&self, cell: &str) -> i32 {
        if cell.chars().next().unwrap().is_ascii_alphabetic() {
            let (x, y) = self.get_pos(cell);
            self.grid[x][y]
        } else {
            cell.parse().unwrap()
        }
    }
}

fn main() {
    let mut spread_sheet = Spreadsheet::new(3);
    assert_eq!(spread_sheet.get_value("=5+7".to_string()), 12);
    spread_sheet.set_cell("A1".to_string(), 10);
    assert_eq!(spread_sheet.get_value("=A1+6".to_string()), 16);
    spread_sheet.set_cell("B2".to_string(), 15);
    assert_eq!(spread_sheet.get_value("=A1+B2".to_string()), 25);
    spread_sheet.reset_cell("A1".to_string());
    assert_eq!(spread_sheet.get_value("=A1+B2".to_string()), 15);
}
