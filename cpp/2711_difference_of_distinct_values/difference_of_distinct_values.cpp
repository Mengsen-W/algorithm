#include <cassert>
#include <set>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> differenceOfDistinctValues(vector<vector<int>>& grid) {
    int m = grid.size();
    int n = grid[0].size();
    vector<vector<int>> res(m, vector<int>(n, 0));

    for (int i = 0; i < m; ++i) {
      int x = i, y = 0;
      set<int> s;
      while (x < m && y < n) {
        res[x][y] += s.size();
        s.insert(grid[x][y]);
        x += 1;
        y += 1;
      }
    }

    for (int j = 1; j < n; ++j) {
      int x = 0, y = j;
      set<int> s;
      while (x < m && y < n) {
        res[x][y] += s.size();
        s.insert(grid[x][y]);
        x += 1;
        y += 1;
      }
    }

    for (int i = 0; i < m; ++i) {
      int x = i, y = n - 1;
      set<int> s;
      while (x >= 0 && y >= 0) {
        res[x][y] -= s.size();
        res[x][y] = abs(res[x][y]);
        s.insert(grid[x][y]);
        x -= 1;
        y -= 1;
      }
    }

    for (int j = n - 2; j >= 0; --j) {
      int x = m - 1, y = j;
      set<int> s;
      while (x >= 0 && y >= 0) {
        res[x][y] -= s.size();
        res[x][y] = abs(res[x][y]);
        s.insert(grid[x][y]);
        x -= 1;
        y -= 1;
      }
    }

    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>>> tests{
      {{{1, 2, 3}, {3, 1, 5}, {3, 2, 1}}, {{1, 1, 0}, {1, 0, 1}, {0, 1, 1}}},
      {{{1}}, {{0}}},
  };

  for (auto& [grid, expected] : tests) {
    assert(Solution().differenceOfDistinctValues(grid) == expected);
  }
}