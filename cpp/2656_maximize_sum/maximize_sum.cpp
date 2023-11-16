/*
 * @Date: 2023-11-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-16
 * @FilePath: /algorithm/cpp/2656_maximize_sum/maximize_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximizeSum(vector<int>& nums, int k) {
    int m = *max_element(nums.begin(), nums.end());
    return (2 * m + k - 1) * k / 2;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 3, 4, 5}, 3, 18},
      {{5, 5, 5}, 2, 11},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().maximizeSum(nums, k) == ans);
  }
}