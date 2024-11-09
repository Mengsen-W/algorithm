#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class NeighborSum {
 public:
  NeighborSum(vector<vector<int>>& grid) {
    for (int i = 0; i < grid.size(); ++i) {
      for (int j = 0; j < grid[0].size(); ++j) {
        pos[grid[i][j]] = {i, j};
      }
    }
    this->grid = std::move(grid);
  }

  int adjacentSum(int value) { return getSum(value, 0); }

  int diagonalSum(int value) { return getSum(value, 1); }

  int getSum(int value, int idx) {
    auto [x, y] = pos[value];
    int ans = 0;
    for (int d = 0; d < 4; ++d) {
      int nx = x + dirs[idx][d][0];
      int ny = y + dirs[idx][d][1];
      if (0 <= nx && nx < grid.size() && 0 <= ny && ny < grid[0].size()) {
        ans += grid[nx][ny];
      }
    }
    return ans;
  }

 private:
  vector<vector<int>> grid;
  unordered_map<int, pair<int, int>> pos;
  static constexpr int dirs[2][4][2] = {{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}, {{-1, -1}, {-1, 1}, {1, -1}, {1, 1}}};
};

int main() {
  {
    vector<vector<int>> input{{0, 1, 2}, {3, 4, 5}, {6, 7, 8}};
    NeighborSum* n = new NeighborSum(input);
    assert(n->adjacentSum(1) == 6);
    assert(n->adjacentSum(4) == 16);
    assert(n->diagonalSum(4) == 16);
    assert(n->diagonalSum(8) == 4);
  }

  {
    vector<vector<int>> input{{{1, 2, 0, 3}, {4, 7, 15, 6}, {8, 9, 10, 11}, {12, 13, 14, 5}}};
    NeighborSum* n = new NeighborSum(input);
    assert(n->adjacentSum(15) == 23);
    assert(n->diagonalSum(9) == 45);
  }
}