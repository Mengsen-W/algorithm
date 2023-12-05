/*
 * @Date: 2023-12-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-05
 * @FilePath: /algorithm/cpp/2477_minimum_fuel_cost/minimum_fuel_cost.cpp
 */

#include <cassert>
#include <functional>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long minimumFuelCost(vector<vector<int>>& roads, int seats) {
    int n = roads.size();
    vector<vector<int>> g(n + 1);
    for (auto& e : roads) {
      g[e[0]].push_back(e[1]);
      g[e[1]].push_back(e[0]);
    }
    long long res = 0;
    function<int(int, int)> dfs = [&](int cur, int fa) -> int {
      int peopleSum = 1;
      for (auto ne : g[cur]) {
        if (ne != fa) {
          int peopleCnt = dfs(ne, cur);
          peopleSum += peopleCnt;
          res += (peopleCnt + seats - 1) / seats;
        }
      }
      return peopleSum;
    };
    dfs(0, -1);
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, long long>> tests{
      {{{0, 1}, {0, 2}, {0, 3}}, 5, 3},
      {{{3, 1}, {3, 2}, {1, 0}, {0, 4}, {0, 5}, {4, 6}}, 2, 7},
      {{}, 1, 0},
  };

  for (auto& [roads, seats, ans] : tests) {
    assert(Solution().minimumFuelCost(roads, seats) == ans);
  }
}