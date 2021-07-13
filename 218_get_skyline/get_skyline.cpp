/*
 * @Date: 2021-07-13 08:36:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-13 08:47:49
 */

#include <algorithm>
#include <cassert>
#include <queue>
#include <vector>
using namespace std;

vector<vector<int>> getSkyline(vector<vector<int>>& buildings) {
  auto cmp = [](const pair<int, int>& a, const pair<int, int>& b) -> bool {
    return a.second < b.second;
  };
  priority_queue<pair<int, int>, vector<pair<int, int>>, decltype(cmp)> que(
      cmp);

  vector<int> boundaries;
  for (auto& building : buildings) {
    boundaries.emplace_back(building[0]);
    boundaries.emplace_back(building[1]);
  }
  sort(boundaries.begin(), boundaries.end());

  vector<vector<int>> ret;
  int n = buildings.size(), idx = 0;
  for (auto& boundary : boundaries) {
    while (idx < n && buildings[idx][0] <= boundary) {
      que.emplace(buildings[idx][1], buildings[idx][2]);
      idx++;
    }
    while (!que.empty() && que.top().first <= boundary) {
      que.pop();
    }

    int maxn = que.empty() ? 0 : que.top().second;
    if (ret.size() == 0 || maxn != ret.back()[1]) {
      ret.push_back({boundary, maxn});
    }
  }
  return ret;
}

int main() {
  {
    vector<vector<int>> buildings{
        {2, 9, 10}, {3, 7, 15}, {5, 12, 12}, {15, 20, 10}, {19, 24, 8}};
    assert(
        getSkyline(buildings) ==
        std::move(vector<vector<int>>{
            {2, 10}, {3, 15}, {7, 12}, {12, 0}, {15, 10}, {20, 8}, {24, 0}}));
  }
  {
    vector<vector<int>> buildings{{0, 2, 3}, {2, 5, 3}};
    assert(getSkyline(buildings) ==
           std::move(vector<vector<int>>{{0, 3}, {5, 0}}));
  }
}