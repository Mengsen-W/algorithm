use std::cmp::min;

struct TextEditor {
    left: Vec<char>,
    right: Vec<char>,
}

impl TextEditor {
    fn new() -> Self {
        TextEditor {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    fn add_text(&mut self, text: String) {
        for c in text.chars() {
            self.left.push(c);
        }
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let count = min(k as usize, self.left.len());
        for _ in 0..count {
            self.left.pop();
        }
        count as i32
    }

    fn cursor_left(&mut self, k: i32) -> String {
        let move_count = min(k as usize, self.left.len());
        for _ in 0..move_count {
            if let Some(c) = self.left.pop() {
                self.right.push(c);
            }
        }
        self.get_left_text()
    }

    fn cursor_right(&mut self, k: i32) -> String {
        let move_count = min(k as usize, self.right.len());
        for _ in 0..move_count {
            if let Some(c) = self.right.pop() {
                self.left.push(c);
            }
        }
        self.get_left_text()
    }

    fn get_left_text(&self) -> String {
        let start = self.left.len().saturating_sub(10);
        self.left[start..].iter().collect()
    }
}

fn main() {
    let mut text_editor = TextEditor::new(); // 当前 text 为 "|" 。（'|' 字符表示光标）
    text_editor.add_text("leetcode".to_string());
    assert_eq!(text_editor.delete_text(4), 4);
    text_editor.add_text("practice".to_string());
    assert_eq!(text_editor.cursor_right(3), "etpractice".to_string());
    assert_eq!(text_editor.cursor_left(8), "leet".to_string());
    assert_eq!(text_editor.delete_text(10), 4);
    assert_eq!(text_editor.cursor_left(2), "".to_string());
    assert_eq!(text_editor.cursor_right(6), "practi".to_string());
}
