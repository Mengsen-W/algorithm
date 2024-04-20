/*
 * @Date: 2024-04-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-20
 * @FilePath: /algorithm/cpp/39_combination_sum/combination_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  void dfs(vector<int>& candidates, int target, vector<vector<int>>& ans, vector<int>& combine, int idx) {
    if (idx == candidates.size()) {
      return;
    }
    if (target == 0) {
      ans.emplace_back(combine);
      return;
    }
    // 直接跳过
    dfs(candidates, target, ans, combine, idx + 1);
    // 选择当前数
    if (target - candidates[idx] >= 0) {
      combine.emplace_back(candidates[idx]);
      dfs(candidates, target - candidates[idx], ans, combine, idx);
      combine.pop_back();
    }
  }

  vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
    vector<vector<int>> ans;
    vector<int> combine;
    dfs(candidates, target, ans, combine, 0);
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<vector<int>>>> tests{
      {{2, 3, 6, 7}, 7, {{2, 2, 3}, {7}}},
      {{2, 3, 5}, 8, {{2, 2, 2, 2}, {2, 3, 3}, {3, 5}}},
      {{2}, 1, {}},
  };

  for (auto& [candidates, target, ans] : tests) {
    assert(Solution().combinationSum(candidates, target) == ans);
  }
}