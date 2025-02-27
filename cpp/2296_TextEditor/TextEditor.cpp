#include <cassert>
#include <string>
#include <vector>

using namespace std;

class TextEditor {
 private:
  vector<char> left;
  vector<char> right;

 public:
  TextEditor() {}

  void addText(string text) {
    for (char c : text) {
      left.push_back(c);
    }
  }

  int deleteText(int k) {
    int n = 0;
    for (; !left.empty() && k > 0; k--) {
      left.pop_back();
      n++;
    }
    return n;
  }

  string cursorLeft(int k) {
    while (!left.empty() && k > 0) {
      right.push_back(left.back());
      left.pop_back();
      k--;
    }
    int n = left.size();
    return string(left.begin() + max(0, n - 10), left.end());
  }

  string cursorRight(int k) {
    while (!right.empty() && k > 0) {
      left.push_back(right.back());
      right.pop_back();
      k--;
    }
    int n = left.size();
    return string(left.begin() + max(0, n - 10), left.end());
  }
};

int main() {
  TextEditor textEditor = TextEditor();
  textEditor.addText("leetcode");
  assert(textEditor.deleteText(4) == 4);
  textEditor.addText("practice");
  assert(textEditor.cursorRight(3) == "etpractice");
  assert(textEditor.cursorLeft(8) == "leet");
  assert(textEditor.deleteText(10) == 4);
  assert(textEditor.cursorLeft(2) == "");
  assert(textEditor.cursorRight(6) == "practi");
}