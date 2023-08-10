/*
 * @Date: 2023-08-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-10
 * @FilePath: /algorithm/cpp/1289_min_falling_path_sum/min_falling_path_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minFallingPathSum(vector<vector<int>>& grid) {
    int n = grid.size();
    int first_min_sum = 0;
    int second_min_sum = 0;
    int first_min_index = -1;

    for (int i = 0; i < n; i++) {
      int cur_first_min_sum = INT_MAX;
      int cur_second_min_sum = INT_MAX;
      int cur_first_min_index = -1;

      for (int j = 0; j < n; j++) {
        int cur_sum = (j != first_min_index ? first_min_sum : second_min_sum) + grid[i][j];
        if (cur_sum < cur_first_min_sum) {
          cur_second_min_sum = cur_first_min_sum;
          cur_first_min_sum = cur_sum;
          cur_first_min_index = j;
        } else if (cur_sum < cur_second_min_sum) {
          cur_second_min_sum = cur_sum;
        }
      }
      first_min_sum = cur_first_min_sum;
      second_min_sum = cur_second_min_sum;
      first_min_index = cur_first_min_index;
    }
    return first_min_sum;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}, 13},
      {{{7}}, 7},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().minFallingPathSum(grid) == ans);
  }
}