/*
 * @Date: 2021-05-02 09:24:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-02 09:42:07
 */

#include <cassert>
#include <unordered_map>
#include <vector>
using namespace std;

int least_bricks(vector<vector<int>>& wall) {
  unordered_map<int, int> cnt;
  for (auto& widths : wall) {
    int n = widths.size();
    int sum = 0;
    for (int i = 0; i < n - 1; i++) {
      sum += widths[i];
      cnt[sum]++;
    }
  }
  int maxCnt = 0;
  for (auto& [_, c] : cnt) {
    maxCnt = max(maxCnt, c);
  }
  return wall.size() - maxCnt;
}

int main() {
  {
    vector<vector<int>> nums{{1, 2, 2, 1}, {3, 1, 2}, {1, 3, 2},
                             {2, 4},       {3, 1, 2}, {1, 3, 1, 1}};
    assert(least_bricks(nums) == 2);
  }
  {
    vector<vector<int>> nums{
        {1},
        {1},
        {1},
    };
    assert(least_bricks(nums) == 3);
  }
}