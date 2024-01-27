/*
 * @Date: 2024-01-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-27
 * @FilePath: /algorithm/cpp/2861_max_number_of_alloys/max_number_of_alloys.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxNumberOfAlloys(int n, int k, int budget, vector<vector<int>>& composition, vector<int>& stock,
                        vector<int>& cost) {
    int left = 1, right = 2e8, ans = 0;
    while (left <= right) {
      int mid = (left + right) / 2;
      bool valid = false;
      for (int i = 0; i < k; ++i) {
        long long spend = 0;
        for (int j = 0; j < n; ++j) {
          spend += max(static_cast<long long>(composition[i][j]) * mid - stock[j], 0LL) * cost[j];
        }
        if (spend <= budget) {
          valid = true;
          break;
        }
      }
      if (valid) {
        ans = mid;
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, int, int, vector<vector<int>>, vector<int>, vector<int>, int>> tests{
      {3, 2, 15, {{1, 1, 1}, {1, 1, 10}}, {0, 0, 0}, {1, 2, 3}, 2},
      {3, 2, 15, {{1, 1, 1}, {1, 1, 10}}, {0, 0, 100}, {1, 2, 3}, 5},
      {2, 3, 10, {{2, 1}, {1, 2}, {1, 1}}, {1, 1}, {5, 5}, 2},
  };

  for (auto& [n, k, budget, composition, stock, cost, ans] : tests) {
    assert(Solution().maxNumberOfAlloys(n, k, budget, composition, stock, cost) == ans);
  }
}