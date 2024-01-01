/*
 * @Date: 2023-03-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-01
 * @FilePath: /algorithm/cpp/1599_min_operations_max_profit/min_operations_max_profit.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperationsMaxProfit(vector<int>& customers, int boardingCost, int runningCost) {
    int ans = -1;
    int maxProfit = 0;
    int totalProfit = 0;
    int operations = 0;
    int customersCount = 0;
    int n = customers.size();
    for (int i = 0; i < n; i++) {
      operations++;
      customersCount += customers[i];
      int curCustomers = min(customersCount, 4);
      customersCount -= curCustomers;
      totalProfit += boardingCost * curCustomers - runningCost;
      if (totalProfit > maxProfit) {
        maxProfit = totalProfit;
        ans = operations;
      }
    }
    if (customersCount == 0) {
      return ans;
    }
    int profitEachTime = boardingCost * 4 - runningCost;
    if (profitEachTime <= 0) {
      return ans;
    }
    if (customersCount > 0) {
      int fullTimes = customersCount / 4;
      totalProfit += profitEachTime * fullTimes;
      operations += fullTimes;
      if (totalProfit > maxProfit) {
        maxProfit = totalProfit;
        ans = operations;
      }
      int remainingCustomers = customersCount % 4;
      int remainingProfit = boardingCost * remainingCustomers - runningCost;
      totalProfit += remainingProfit;
      if (totalProfit > maxProfit) {
        maxProfit = totalProfit;
        operations++;
        ans++;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, int>> tests{
      {{8, 3}, 5, 6, 3},
      {{10, 9, 6}, 6, 4, 7},
      {{3, 4, 0, 5, 1}, 1, 92, -1},
  };

  for (auto& [customers, boardingCost, runningCost, ans] : tests) {
    assert(Solution().minOperationsMaxProfit(customers, boardingCost, runningCost) == ans);
  }
}