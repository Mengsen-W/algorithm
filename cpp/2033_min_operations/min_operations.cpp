#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<vector<int>>& grid, int x) {
    vector<int> nums;
    int m = grid.size(), n = grid[0].size();
    for (int i = 0; i < m; ++i) {
      for (int j = 0; j < n; ++j) {
        if ((grid[i][j] - grid[0][0]) % x != 0) {
          return -1;
        }
        nums.push_back(grid[i][j]);
      }
    }

    sort(nums.begin(), nums.end());
    int choose = nums[nums.size() / 2];
    int ans = 0;
    for (int num : nums) {
      ans += abs(num - choose) / x;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {{{2, 4}, {6, 8}}, 2, 4},
      {{{1, 5}, {2, 3}}, 1, 5},
      {{{1, 2}, {3, 4}}, 2, -1},
  };

  for (auto [grid, x, expected] : tests) {
    assert(Solution().minOperations(grid, x) == expected);
  }
}