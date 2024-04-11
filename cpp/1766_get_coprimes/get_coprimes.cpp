/*
 * @Date: 2024-04-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-11
 * @FilePath: /algorithm/cpp/1766_get_coprimes/get_coprimes.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 private:
  vector<vector<int>> gcds;
  vector<vector<int>> tmp;
  vector<vector<int>> g;
  vector<int> dep;
  vector<int> ans;

 public:
  void dfs(vector<int>& nums, int x, int depth) {
    dep[x] = depth;
    for (int val : gcds[nums[x]]) {
      if (tmp[val].empty()) {
        continue;
      }

      int las = tmp[val].back();
      if (ans[x] == -1 || dep[las] > dep[ans[x]]) {
        ans[x] = las;
      }
    }
    tmp[nums[x]].push_back(x);

    for (int val : g[x]) {
      if (dep[val] == -1) {  // 被访问过的点dep不为-1
        dfs(nums, val, depth + 1);
      }
    }

    tmp[nums[x]].pop_back();
  }

  vector<int> getCoprimes(vector<int>& nums, vector<vector<int>>& edges) {
    int n = nums.size();

    // 初始化
    gcds.resize(51);
    tmp.resize(51);
    ans.resize(n, -1);
    dep.resize(n, -1);
    g.resize(n);

    for (int i = 1; i <= 50; i++) {
      for (int j = 1; j <= 50; j++) {
        if (gcd(i, j) == 1) {
          gcds[i].push_back(j);
        }
      }
    }

    for (const auto& val : edges) {
      g[val[0]].push_back(val[1]);
      g[val[1]].push_back(val[0]);
    }

    dfs(nums, 0, 1);

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>, vector<int>>> tests{
      {{2, 3, 3, 2}, {{0, 1}, {1, 2}, {1, 3}}, {-1, 0, 0, 1}},
      {
          {5, 6, 10, 2, 3, 6, 15},
          {{0, 1}, {0, 2}, {1, 3}, {1, 4}, {2, 5}, {2, 6}},
          {-1, 0, -1, 0, 0, 0, -1},
      },
  };

  for (auto& [nums, edges, ans] : tests) {
    assert(Solution().getCoprimes(nums, edges) == ans);
  }
}