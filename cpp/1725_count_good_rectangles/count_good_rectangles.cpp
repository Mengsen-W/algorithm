/*
 * @Date: 2022-02-04 00:55:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-04 01:00:52
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int countGoodRectangles(vector<vector<int>> rectangles) {
    int res = 0, maxLen = 0;
    for (auto& rectangle : rectangles) {
      int l = rectangle[0], w = rectangle[1];
      int k = min(l, w);
      if (k == maxLen) {
        ++res;
      } else if (k > maxLen) {
        res = 1;
        maxLen = k;
      }
    }
    return res;
  }
};

int main() {
  assert(Solution().countGoodRectangles(
             vector<vector<int>>{{5, 8}, {3, 9}, {5, 12}, {16, 5}}) == 3);

  assert(Solution().countGoodRectangles(
             vector<vector<int>>{{2, 3}, {3, 7}, {4, 3}, {3, 7}}) == 3);
}