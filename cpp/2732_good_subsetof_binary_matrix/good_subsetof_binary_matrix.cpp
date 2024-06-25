#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> goodSubsetofBinaryMatrix(vector<vector<int>>& grid) {
    vector<int> ans;
    unordered_map<int, int> mp;
    int m = grid.size();
    int n = grid[0].size();

    for (int j = 0; j < m; j++) {
      int st = 0;
      for (int i = 0; i < n; i++) {
        st |= (grid[j][i] << i);
      }
      mp[st] = j;
    }

    if (mp.count(0)) {
      ans.push_back(mp[0]);
      return ans;
    }

    for (auto [x, i] : mp) {
      for (auto [y, j] : mp) {
        if (!(x & y)) {
          return {min(i, j), max(i, j)};
        }
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>>> tests{
      {{{0, 1, 1, 0}, {0, 0, 0, 1}, {1, 1, 1, 1}}, {0, 1}},
      {{{0}}, {0}},
      {{{1, 1, 1}, {1, 1, 1}}, {}},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().goodSubsetofBinaryMatrix(grid) == ans);
  }
}