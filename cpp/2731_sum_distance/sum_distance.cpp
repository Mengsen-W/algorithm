/*
 * @Date: 2023-10-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-10
 * @FilePath: /algorithm/cpp/2731_sum_distance/sum_distance.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  static constexpr int mod = 1e9 + 7;
  int sumDistance(vector<int>& nums, string s, int d) {
    int n = nums.size();
    vector<long long> pos(n);
    for (int i = 0; i < n; i++) {
      if (s[i] == 'L') {
        pos[i] = (long long)nums[i] - d;
      } else {
        pos[i] = (long long)nums[i] + d;
      }
    }
    sort(pos.begin(), pos.end());
    long long res = 0;
    for (int i = 1; i < n; i++) {
      res += 1ll * (pos[i] - pos[i - 1]) * i % mod * (n - i) % mod;
      res %= mod;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, string, int, int>> tests{
      {{-2, 0, 2}, "RLL", 3, 8},
      {{1, 0}, "RL", 2, 5},
  };

  for (auto& [nums, s, d, ans] : tests) {
    assert(Solution().sumDistance(nums, s, d) == ans);
  }
}