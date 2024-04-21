/*
 * @Date: 2024-04-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-21
 * @FilePath: /algorithm/cpp/216_combination_sum3/combination_sum3.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> temp;
  vector<vector<int>> ans;

  void dfs(int cur, int n, int k, int sum) {
    if (temp.size() + (n - cur + 1) < k || temp.size() > k) {
      return;
    }
    if (temp.size() == k && accumulate(temp.begin(), temp.end(), 0) == sum) {
      ans.push_back(temp);
      return;
    }
    temp.push_back(cur);
    dfs(cur + 1, n, k, sum);
    temp.pop_back();
    dfs(cur + 1, n, k, sum);
  }

  vector<vector<int>> combinationSum3(int k, int n) {
    dfs(1, 9, k, n);
    return ans;
  }
};

int main() {
  vector<tuple<int, int, vector<vector<int>>>> tests{
      {3, 7, {{1, 2, 4}}},
      {3, 9, {{1, 2, 6}, {1, 3, 5}, {2, 3, 4}}},
      {4, 1, {}},
  };

  for (auto& [k, n, ans] : tests) {
    assert(Solution().combinationSum3(k, n) == ans);
  }
}