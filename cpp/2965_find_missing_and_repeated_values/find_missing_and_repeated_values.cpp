#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findMissingAndRepeatedValues(vector<vector<int>>& grid) {
    int n = grid.size();
    vector<int> count(n * n + 1);
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < n; j++) {
        count[grid[i][j]]++;
      }
    }

    vector<int> res(2);
    for (int i = 1; i <= n * n; i++) {
      if (count[i] == 2) {
        res[0] = i;
      }
      if (count[i] == 0) {
        res[1] = i;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>>> tests{
      {{{1, 3}, {2, 2}}, {2, 4}},
      {{{9, 1, 7}, {8, 9, 2}, {3, 4, 6}}, {9, 5}},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().findMissingAndRepeatedValues(grid) == ans);
  }
}