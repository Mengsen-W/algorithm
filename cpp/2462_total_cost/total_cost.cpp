/*
 * @Date: 2024-05-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-01
 * @FilePath: /algorithm/cpp/2462_total_cost/total_cost.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long totalCost(vector<int>& costs, int k, int candidates) {
    int n = costs.size();
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> q;
    int left = candidates - 1, right = n - candidates;
    if (left + 1 < right) {
      for (int i = 0; i <= left; ++i) {
        q.emplace(costs[i], i);
      }
      for (int i = right; i < n; ++i) {
        q.emplace(costs[i], i);
      }
    } else {
      for (int i = 0; i < n; ++i) {
        q.emplace(costs[i], i);
      }
    }
    long long ans = 0;
    for (int _ = 0; _ < k; ++_) {
      auto [cost, id] = q.top();
      q.pop();
      ans += cost;
      if (left + 1 < right) {
        if (id <= left) {
          ++left;
          q.emplace(costs[left], left);
        } else {
          --right;
          q.emplace(costs[right], right);
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, long long>> tests{
      {{17, 12, 10, 2, 7, 2, 11, 20, 8}, 3, 4, 11},
      {{1, 2, 4, 1}, 3, 3, 4},
  };

  for (auto& [costs, k, candidates, ans] : tests) {
    assert(Solution().totalCost(costs, k, candidates) == ans);
  }
}