#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Spreadsheet {
 public:
  Spreadsheet(int rows) { this->grid = vector<vector<int>>(rows + 1, vector<int>(26)); }

  void setCell(string cell, int value) {
    auto [x, y] = getPos(cell);
    grid[x][y] = value;
  }

  void resetCell(string cell) {
    auto [x, y] = getPos(cell);
    grid[x][y] = 0;
  }

  int getValue(string formula) {
    int i = formula.find('+');
    string cell1 = formula.substr(1, i - 1);
    string cell2 = formula.substr(i + 1);
    return getCellVal(cell1) + getCellVal(cell2);
  }

 private:
  vector<vector<int>> grid;

  pair<int, int> getPos(const string &cell) {
    int x = stoi(cell.substr(1));
    int y = cell[0] - 'A';
    return {x, y};
  }

  int getCellVal(string &cell) {
    if (isalpha(cell[0])) {
      auto [x, y] = getPos(cell);
      return grid[x][y];
    } else {
      return stoi(cell);
    }
  }
};

int main() {
  Spreadsheet spreadsheet = Spreadsheet(3);
  assert(spreadsheet.getValue("=5+7") == 12);    // 返回 12 (5+7)
  spreadsheet.setCell("A1", 10);                 // 设置 A1 为 10
  assert(spreadsheet.getValue("=A1+6") == 16);   // 返回 16 (10+6)
  spreadsheet.setCell("B2", 15);                 // 设置 B2 为 15
  assert(spreadsheet.getValue("=A1+B2") == 25);  // 返回 25 (10+15)
  spreadsheet.resetCell("A1");                   // 重置 A1 为 0
  assert(spreadsheet.getValue("=A1+B2") == 15);  // 返回 15 (0+15)
}