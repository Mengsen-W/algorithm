/*
 * @Date: 2023-08-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-22
 * @FilePath: /algorithm/cpp/849_max_dist_to_closest/max_dist_to_closest.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxDistToClosest(vector<int>& seats) {
    int res = 0;
    int l = 0;
    // 找到最左侧有人的位置
    while (l < seats.size() && seats[l] == 0) {
      ++l;
    }
    // 左侧是空的情况
    res = max(res, l);
    while (l < seats.size()) {
      int r = l + 1;
      // 找到右侧位置
      while (r < seats.size() && seats[r] == 0) {
        ++r;
      }

      if (r == seats.size()) {  // 右侧是空的情况
        res = max(res, r - l - 1);
      } else {  // 左右都有人的情况
        res = max(res, (r - l) / 2);
      }

      // 遍历到下一轮
      l = r;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 0, 0, 0, 1, 0, 1}, 2},
      {{1, 0, 0, 0}, 3},
      {{0, 1}, 1},
  };

  for (auto& [seats, range] : tests) {
    assert(Solution().maxDistToClosest(seats) == range);
  }
}