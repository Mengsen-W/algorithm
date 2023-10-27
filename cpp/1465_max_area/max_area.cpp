/*
 * @Date: 2023-10-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-27
 * @FilePath: /algorithm/cpp/1465_max_area/max_area.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxArea(int h, int w, vector<int>& horizontalCuts, vector<int>& verticalCuts) {
    int mod = 1e9 + 7;
    sort(horizontalCuts.begin(), horizontalCuts.end());
    sort(verticalCuts.begin(), verticalCuts.end());
    auto calMax = [](vector<int>& arr, int boardr) -> int {
      int res = 0, pre = 0;
      for (int i : arr) {
        res = max(i - pre, res);
        pre = i;
      }
      return max(res, boardr - pre);
    };
    return (long long)calMax(horizontalCuts, h) * calMax(verticalCuts, w) % mod;
  }
};

int main() {
  vector<tuple<int, int, vector<int>, vector<int>, int>> tests{
      {5, 4, {1, 2, 4}, {1, 3}, 4},
      {5, 4, {3, 1}, {1}, 6},
  };

  for (auto& [h, w, horizontalCuts, verticalCuts, ans] : tests) {
    assert(Solution().maxArea(h, w, horizontalCuts, verticalCuts) == ans);
  }
}