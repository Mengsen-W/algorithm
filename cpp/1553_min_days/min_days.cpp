/*
 * @Date: 2024-05-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-12
 * @FilePath: /algorithm/cpp/1553_min_days/min_days.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

using TIII = tuple<int, int, int>;

class Solution {
 public:
  int minDays(int n) {
    auto getHeuristicValue = [](int rest) -> int {
      return rest == 0 ? 0 : static_cast<int>(log(static_cast<double>(rest)) / log(3.)) + 1;
    };
    auto compareFn = [](const TIII& u, const TIII& v) { return get<0>(u) + get<1>(u) > get<0>(v) + get<1>(v); };
    priority_queue<TIII, vector<TIII>, decltype(compareFn)> q(compareFn);
    unordered_set<int> visited;
    q.emplace(0, getHeuristicValue(n), n);
    int ans = 0;
    while (true) {
      auto [days, heuristic, rest] = q.top();
      q.pop();
      if (visited.count(rest)) {
        continue;
      }
      visited.insert(rest);
      if (rest == 1) {
        ans = days + 1;
        break;
      }
      q.emplace(days + rest % 2 + 1, getHeuristicValue(rest / 2), rest / 2);
      q.emplace(days + rest % 3 + 1, getHeuristicValue(rest / 3), rest / 3);
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, int>> tests{
      {10, 4},
      {6, 3},
      {1, 1},
      {56, 6},
  };

  for (auto& [n, ans] : tests) {
    assert(Solution().minDays(n) == ans);
  }
}