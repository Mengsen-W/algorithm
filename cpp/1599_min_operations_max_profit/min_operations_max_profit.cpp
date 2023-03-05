/*
 * @Date: 2023-03-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-05
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
  {
    vector<int> customers{8, 3};
    int boardingCost = 5;
    int runningCost = 6;
    int ans = 3;
    assert(Solution().minOperationsMaxProfit(customers, boardingCost, runningCost) == ans);
  }

  {
    vector<int> customers{10,9,6};
    int boardingCost = 6;
    int runningCost = 4;
    int ans = 7;
    assert(Solution().minOperationsMaxProfit(customers, boardingCost, runningCost) == ans);
  }

  {
    vector<int> customers{3,4,0,5,1};
    int boardingCost = 1;
    int runningCost = 92;
    int ans = -1;
    assert(Solution().minOperationsMaxProfit(customers, boardingCost, runningCost) == ans);
  }
}