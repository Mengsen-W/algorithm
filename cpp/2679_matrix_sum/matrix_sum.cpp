/*
 * @Date: 2023-07-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-04
 * @FilePath: /algorithm/cpp/2679_matrix_sum/matrix_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int matrixSum(vector<vector<int>>& nums) {
    int res = 0;
    int m = nums.size();
    int n = nums[0].size();
    for (int i = 0; i < m; i++) {
      sort(nums[i].begin(), nums[i].end());
    }
    for (int j = 0; j < n; j++) {
      int maxVal = 0;
      for (int i = 0; i < m; i++) {
        maxVal = max(maxVal, nums[i][j]);
      }
      res += maxVal;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> test_cases{
      {vector<vector<int>>{{7, 2, 1}, {6, 4, 2}, {6, 5, 3}, {3, 2, 1}}, 15},
      {vector<vector<int>>{{1}}, 1},
  };

  for (auto& [nums, expected] : test_cases) {
    assert(Solution().matrixSum(nums) == expected);
  }
}