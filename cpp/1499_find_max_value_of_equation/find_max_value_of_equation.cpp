/*
 * @Date: 2023-07-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-21
 * @FilePath: /algorithm/cpp/1499_find_max_value_of_equation/find_max_value_of_equation.cpp
 */

#include <algorithm>
#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  using pii = pair<int, int>;
  int findMaxValueOfEquation(vector<vector<int>>& points, int k) {
    int res = INT_MIN;
    deque<pii> qu;
    for (auto& point : points) {
      int x = point[0], y = point[1];
      while (!qu.empty() && x - qu.front().second > k) {
        qu.pop_front();
      }
      if (!qu.empty()) {
        res = max(res, x + y + qu.front().first);
      }
      while (!qu.empty() && y - x >= qu.back().first) {
        qu.pop_back();
      }
      qu.emplace_back(y - x, x);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {{{1, 3}, {2, 0}, {5, 10}, {6, -10}}, 1, 4},
      {{{0, 0}, {3, 0}, {9, 2}}, 3, 3},
  };
  for (auto& [points, k, ans] : tests) {
    assert(Solution().findMaxValueOfEquation(points, k) == ans);
  }
}
