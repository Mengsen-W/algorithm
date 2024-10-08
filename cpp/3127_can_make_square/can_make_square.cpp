#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canMakeSquare(vector<vector<char>>& grid) {
    for (int i = 0; i <= 1; i++) {
      for (int j = 0; j <= 1; j++) {
        if (check(grid, i, j)) {
          return true;
        }
      }
    }
    return false;
  }
  bool check(vector<vector<char>>& grid, int x, int y) {
    int count = 0;
    for (int i = 0; i <= 1; i++) {
      for (int j = 0; j <= 1; j++) {
        count += (grid[x + i][y + j] == 'B');
      }
    }
    return count != 2;
  }
};

int main() {
  vector<tuple<vector<vector<char>>, bool>> tests{
      {{{'B', 'W', 'B'}, {'B', 'W', 'W'}, {'B', 'W', 'B'}}, true},
      {{{'B', 'W', 'B'}, {'W', 'B', 'W'}, {'B', 'W', 'B'}}, false},
      {{{'B', 'W', 'B'}, {'B', 'W', 'W'}, {'B', 'W', 'W'}}, true},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().canMakeSquare(grid) == ans);
  }
}