/*
 * @Date: 2024-02-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-29
 * @FilePath: /algorithm/cpp/2581_root_count/root_count.cpp
 */

#include <cassert>
#include <functional>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  using ll = long long;
  int rootCount(vector<vector<int>> &edges, vector<vector<int>> &guesses, int k) {
    int n = edges.size() + 1;
    vector<vector<int>> g(n);
    unordered_set<ll> st;
    for (auto &v : edges) {
      g[v[0]].push_back(v[1]);
      g[v[1]].push_back(v[0]);
    }
    auto h = [&](int x, int y) -> ll { return (ll)x << 20 | y; };
    for (auto &v : guesses) {
      st.insert(h(v[0], v[1]));
    }

    int cnt = 0, res = 0;
    function<void(int, int)> dfs = [&](int x, int fat) -> void {
      for (auto &y : g[x]) {
        if (y == fat) {
          continue;
        }
        cnt += st.count(h(x, y));
        dfs(y, x);
      }
    };
    dfs(0, -1);

    function<void(int, int, int)> redfs = [&](int x, int fat, int cnt) {
      if (cnt >= k) {
        res++;
      }
      for (auto &y : g[x]) {
        if (y == fat) {
          continue;
        }
        redfs(y, x, cnt - st.count(h(x, y)) + st.count(h(y, x)));
      }
    };
    redfs(0, -1, cnt);
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>, int, int>> tests{
      {{{0, 1}, {1, 2}, {1, 3}, {4, 2}}, {{1, 3}, {0, 1}, {1, 0}, {2, 4}}, 3, 3},
      {{{0, 1}, {1, 2}, {2, 3}, {3, 4}}, {{1, 0}, {3, 4}, {2, 1}, {3, 2}}, 1, 5},
  };

  for (auto &[edges, guesses, k, ans] : tests) {
    assert(Solution().rootCount(edges, guesses, k) == ans);
  }
}