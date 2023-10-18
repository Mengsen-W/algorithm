/*
 * @Date: 2023-10-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-18
 * @FilePath: /algorithm/cpp/2530_max_kelements/max_kelements.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maxKelements(vector<int>& nums, int k) {
    priority_queue<int> q(nums.begin(), nums.end());
    long long ans = 0;
    for (int _ = 0; _ < k; ++_) {
      int x = q.top();
      q.pop();
      ans += x;
      q.push((x + 2) / 3);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{10, 10, 10, 10, 10}, 5, 50},
      {{1, 10, 3, 3, 3}, 3, 17},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().maxKelements(nums, k) == ans);
  }
}