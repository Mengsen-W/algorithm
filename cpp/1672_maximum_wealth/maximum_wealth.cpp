/*
 * @Date: 2022-04-14 09:16:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-14 09:33:45
 * @FilePath: /algorithm/1672_maximum_wealth/maximum_wealth.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumWealth(vector<vector<int>> accounts) {
    int maxWealth = INT_MIN;
    for (auto& account : accounts) {
      maxWealth = max(maxWealth, accumulate(account.begin(), account.end(), 0));
    }
    return maxWealth;
  }
};

int main() {
  assert(Solution().maximumWealth(vector<vector<int>>{{1, 2, 3}, {3, 2, 1}}) == 6);
  assert(Solution().maximumWealth(vector<vector<int>>{{1, 5}, {7, 3}, {3, 5}}) == 10);
  assert(Solution().maximumWealth(vector<vector<int>>{{2, 8, 7}, {7, 1, 3}, {1, 9, 5}}) == 17);
}