/*
 * @Date: 2023-01-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2023-01-02
 * @FilePath: /algorithm/1801_get_number_of_backlog_orders/get_number_of_backlog_orders.cpp
 */

#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  int getNumberOfBacklogOrders(vector<vector<int>>& orders) {
    const int MOD = 1000000007;
    priority_queue<pair<int, int>, vector<pair<int, int>>, less<pair<int, int>>> buyOrders;
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> sellOrders;
    for (auto&& order : orders) {
      int price = order[0], amount = order[1], orderType = order[2];
      if (orderType == 0) {
        while (amount > 0 && !sellOrders.empty() && sellOrders.top().first <= price) {
          auto sellOrder = sellOrders.top();
          sellOrders.pop();
          int sellAmount = min(amount, sellOrder.second);
          amount -= sellAmount;
          sellOrder.second -= sellAmount;
          if (sellOrder.second > 0) {
            sellOrders.push(sellOrder);
          }
        }
        if (amount > 0) {
          buyOrders.emplace(price, amount);
        }
      } else {
        while (amount > 0 && !buyOrders.empty() && buyOrders.top().first >= price) {
          auto buyOrder = buyOrders.top();
          buyOrders.pop();
          int buyAmount = min(amount, buyOrder.second);
          amount -= buyAmount;
          buyOrder.second -= buyAmount;
          if (buyOrder.second > 0) {
            buyOrders.push(buyOrder);
          }
        }
        if (amount > 0) {
          sellOrders.emplace(price, amount);
        }
      }
    }
    int total = 0;
    while (!buyOrders.empty()) {
      total = (total + buyOrders.top().second) % MOD;
      buyOrders.pop();
    }
    while (!sellOrders.empty()) {
      total = (total + sellOrders.top().second) % MOD;
      sellOrders.pop();
    }
    return total;
  }
};

int main() {
  {
    vector<vector<int>> orders{{10, 5, 0}, {15, 2, 1}, {25, 1, 1}, {30, 4, 0}};
    int ans = 6;
    assert(Solution().getNumberOfBacklogOrders(orders) == ans);
  }

  {
    vector<vector<int>> orders{{7, 1000000000, 1}, {15, 3, 0}, {5, 999999995, 0}, {5, 1, 1}};
    int ans = 999999984;
    assert(Solution().getNumberOfBacklogOrders(orders) == ans);
  }
}