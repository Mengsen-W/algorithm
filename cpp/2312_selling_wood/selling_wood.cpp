/*
 * @Date: 2024-03-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-15
 * @FilePath: /algorithm/cpp/2312_selling_wood/selling_wood.cpp
 */

#include <cassert>
#include <functional>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  long long sellingWood(int m, int n, vector<vector<int>>& prices) {
    auto pair_hash = [fn = hash<int>()](const pair<int, int>& o) -> size_t {
      return (fn(o.first) << 16) ^ fn(o.second);
    };
    unordered_map<pair<int, int>, int, decltype(pair_hash)> value(0, pair_hash);

    vector<vector<long long>> memo(m + 1, vector<long long>(n + 1, -1));

    function<long long(int, int)> dfs = [&](int x, int y) -> long long {
      if (memo[x][y] != -1) {
        return memo[x][y];
      }

      long long ret = value.count({x, y}) ? value[{x, y}] : 0;
      if (x > 1) {
        for (int i = 1; i < x; ++i) {
          ret = max(ret, dfs(i, y) + dfs(x - i, y));
        }
      }
      if (y > 1) {
        for (int j = 1; j < y; ++j) {
          ret = max(ret, dfs(x, j) + dfs(x, y - j));
        }
      }
      return memo[x][y] = ret;
    };

    for (int i = 0; i < prices.size(); ++i) {
      value[{prices[i][0], prices[i][1]}] = prices[i][2];
    }
    return dfs(m, n);
  }
};

int main() {
  vector<tuple<int, int, vector<vector<int>>, long long>> tests{
      {3, 5, {{1, 4, 2}, {2, 2, 7}, {2, 1, 3}}, 19},
      {4, 6, {{3, 2, 10}, {1, 4, 2}, {4, 1, 3}}, 32},
  };

  for (auto& [m, n, prices, ans] : tests) {
    assert(Solution().sellingWood(m, n, prices) == ans);
  }
}