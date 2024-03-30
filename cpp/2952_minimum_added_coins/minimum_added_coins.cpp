/*
 * @Date: 2024-03-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-30
 * @FilePath: /algorithm/cpp/2952_minimum_added_coins/minimum_added_coins.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumAddedCoins(vector<int>& coins, int target) {
    sort(coins.begin(), coins.end());
    int ans = 0;
    int x = 1;
    int length = coins.size(), index = 0;
    while (x <= target) {
      if (index < length && coins[index] <= x) {
        x += coins[index];
        index++;
      } else {
        x <<= 1;
        ans++;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 4, 10}, 19, 2},
      {{1, 4, 10, 5, 7, 19}, 19, 1},
      {{1, 1, 1}, 20, 3},
  };

  for (auto& [coins, target, ans] : tests) {
    assert(Solution().minimumAddedCoins(coins, target) == ans);
  }
}