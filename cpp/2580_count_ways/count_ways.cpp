/*
 * @Date: 2024-03-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-27
 * @FilePath: /algorithm/cpp/2580_count_ways/count_ways.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  static constexpr int mod = 1e9 + 7;
  int countWays(vector<vector<int>>& ranges) {
    sort(ranges.begin(), ranges.end());
    int n = ranges.size();
    long long res = 1;
    for (int i = 0; i < n;) {
      int r = ranges[i][1];
      int j = i + 1;
      while (j < n && ranges[j][0] <= r) {
        r = max(r, ranges[j][1]);
        j++;
      }
      res = res * 2 % mod;
      i = j;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{6, 10}, {5, 15}}, 2},
      {{{1, 3}, {10, 20}, {2, 5}, {4, 8}}, 4},
  };

  for (auto& [ranges, ans] : tests) {
    assert(Solution().countWays(ranges) == ans);
  }
}