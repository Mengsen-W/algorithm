/*
 * @Date: 2023-12-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-28
 * @FilePath: /algorithm/cpp/2735_min_cost/min_cost.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long minCost(vector<int>& nums, int x) {
    int n = nums.size();
    vector<int> f(nums);
    long long ans = accumulate(f.begin(), f.end(), 0LL);
    for (int k = 1; k < n; ++k) {
      for (int i = 0; i < n; ++i) {
        f[i] = min(f[i], nums[(i + k) % n]);
      }
      ans = min(ans, static_cast<long long>(k) * x + accumulate(f.begin(), f.end(), 0LL));
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{20, 1, 15}, 5, 13},
      {{1, 2, 3}, 4, 6},
  };

  for (auto& [nums, x, ans] : tests) {
    assert(Solution().minCost(nums, x) == ans);
  }
}