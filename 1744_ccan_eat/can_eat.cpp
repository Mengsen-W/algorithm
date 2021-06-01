/*
 * @Date: 2021-06-01 09:00:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-01 09:05:36
 */

#include <cassert>
#include <vector>

using namespace std;

using LL = long long;

vector<bool> canEat(vector<int>& candiesCount, vector<vector<int>>& queries) {
  int n = candiesCount.size();

  // 前缀和
  vector<LL> sum(n);
  sum[0] = candiesCount[0];
  for (int i = 1; i < n; ++i) {
    sum[i] = sum[i - 1] + candiesCount[i];
  }

  vector<bool> ans;
  for (const auto& q : queries) {
    int favoriteType = q[0], favoriteDay = q[1], dailyCap = q[2];

    LL x1 = favoriteDay + 1;
    LL y1 = (LL)(favoriteDay + 1) * dailyCap;
    LL x2 = (favoriteType == 0 ? 1 : sum[favoriteType - 1] + 1);
    LL y2 = sum[favoriteType];

    ans.push_back(!(x1 > y2 || y1 < x2));
  }
  return ans;
}

int main() {
  {
    vector<int> candiesCount{7, 4, 5, 3, 8};
    vector<vector<int>> queries{{0, 2, 2}, {4, 2, 4}, {2, 13, 1000000000}};
    vector<bool> ans{true, false, true};
    assert(canEat(candiesCount, queries) == ans);
  }
  {
    vector<int> candiesCount{5, 2, 6, 4, 1};
    vector<vector<int>> queries{
        {3, 1, 2}, {4, 10, 3}, {3, 10, 100}, {4, 100, 30}, {1, 3, 1}};
    vector<bool> ans{false, true, true, false, false};
    assert(canEat(candiesCount, queries) == ans);
  }
}