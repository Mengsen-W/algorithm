/*
 * @Date: 2023-10-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-22
 * @FilePath: /algorithm/cpp/1402_max_satisfaction/max_satisfaction.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxSatisfaction(vector<int>& satisfaction) {
    sort(satisfaction.begin(), satisfaction.end(), greater<int>());
    int presum = 0, ans = 0;
    for (int si : satisfaction) {
      if (presum + si > 0) {
        presum += si;
        ans += presum;
      } else {
        break;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{-1, -8, 0, 5, -9}, 14},
      {{4, 3, 2}, 20},
      {{-1, -4, -5}, 0},
  };

  for (auto& [satisfaction, ans] : tests) {
    assert(Solution().maxSatisfaction(satisfaction) == ans);
  }
}