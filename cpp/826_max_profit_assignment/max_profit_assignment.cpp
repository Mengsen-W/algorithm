/*
 * @Date: 2024-05-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-17
 * @FilePath: /algorithm/cpp/826_max_profit_assignment/max_profit_assignment.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProfitAssignment(vector<int>& difficulty, vector<int>& profit, vector<int>& worker) {
    vector<pair<int, int>> jobs;
    int n = profit.size(), res = 0, i = 0, best = 0;
    for (int j = 0; j < n; ++j) {
      jobs.emplace_back(difficulty[j], profit[j]);
    }
    sort(jobs.begin(), jobs.end());
    sort(worker.begin(), worker.end());
    for (int w : worker) {
      while (i < n && w >= jobs[i].first) {
        best = max(best, jobs[i].second);
        i++;
      }
      res += best;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, vector<int>, int>> tests{
      {{2, 4, 6, 8, 10}, {10, 20, 30, 40, 50}, {4, 5, 6, 7}, 100},
      {{85, 47, 57}, {24, 66, 99}, {40, 25, 25}, 0},
  };

  for (auto& [difficulty, profit, worker, ans] : tests) {
    assert(Solution().maxProfitAssignment(difficulty, profit, worker) == ans);
  }
}