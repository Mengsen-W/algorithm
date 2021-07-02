/*
 * @Date: 2021-07-02 14:57:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-02 15:01:27
 */

#include <algorithm>
#include <cassert>
#include <vector>
using namespace std;

int maxIceCream(vector<int>& costs, int coins) {
  vector<int> freq(100001);
  for (int& cost : costs) {
    freq[cost]++;
  }
  int count = 0;
  for (int i = 1; i <= 100000; i++) {
    if (coins >= i) {
      int curCount = min(freq[i], coins / i);
      count += curCount;
      coins -= i * curCount;
    } else {
      break;
    }
  }
  return count;
}

int main() {
  {
    vector<int> costs{1, 3, 2, 4, 1};
    int coins = 7;
    assert(maxIceCream(costs, coins) == 4);
  }
  {
    vector<int> costs{10, 6, 8, 7, 7, 8};
    int coins = 5;
    assert(maxIceCream(costs, coins) == 0);
  }
  {
    vector<int> costs{1, 6, 3, 1, 2, 5};
    int coins = 20;
    assert(maxIceCream(costs, coins) == 6);
  }
}