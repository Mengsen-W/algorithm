/*
 * @Date: 2023-07-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-28
 * @FilePath: /algorithm/cpp/2050_minimum_time/minimum_time.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>
#include <functional>

using namespace std;

class Solution {
 public:
  int minimumTime(int n, vector<vector<int>>& relations, vector<int>& time) {
    int mx = 0;
    vector<vector<int>> prev(n + 1);
    for (auto& relation : relations) {
      int x = relation[0], y = relation[1];
      prev[y].emplace_back(x);
    }
    unordered_map<int, int> memo;
    function<int(int)> dp = [&](int i) -> int {
      if (!memo.count(i)) {
        int cur = 0;
        for (int p : prev[i]) {
          cur = max(cur, dp(p));
        }
        cur += time[i - 1];
        memo[i] = cur;
      }
      return memo[i];
    };

    for (int i = 1; i <= n; i++) {
      mx = max(mx, dp(i));
    }
    return mx;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>, int>> tests{
      {3, {{1, 3}, {2, 3}}, {2, 3, 5}, 8},
      {5, {{1, 5}, {2, 5}, {3, 5}, {3, 4}, {4, 5}}, {1, 2, 3, 4, 5}, 12},
  };
  for (auto& [n, relations, time, ans] : tests) {
    assert(Solution().minimumTime(n, relations, time) == ans);
  }
}
