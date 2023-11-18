/*
 * @Date: 2023-11-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-18
 * @FilePath: /algorithm/cpp/2342_maximum_sum/maximum_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumSum(vector<int> &nums) {
    int ans = -1;
    int mx[82]{};
    for (int num : nums) {
      int s = 0;
      for (int x = num; x; x /= 10) s += x % 10;
      if (mx[s]) ans = max(ans, mx[s] + num);
      mx[s] = max(mx[s], num);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{18, 43, 36, 13, 7}, 54},
      {{10, 12, 19, 14}, -1},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().maximumSum(nums) == ans);
  }
}