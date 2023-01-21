/*
 * @Date: 2023-01-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-21
 * @FilePath: /algorithm/cpp/1824_min_side_jumps/min_side_jumps.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
  static constexpr int inf = 0x3f3f3f3f;

 public:
  int minSideJumps(vector<int> &obstacles) {
    int n = obstacles.size() - 1;
    vector<int> d = {1, 0, 1};
    for (int i = 1; i <= n; i++) {
      int minCnt = inf;
      for (int j = 0; j < 3; j++) {
        if (j == obstacles[i] - 1) {
          d[j] = inf;
        } else {
          minCnt = min(minCnt, d[j]);
        }
      }
      for (int j = 0; j < 3; j++) {
        if (j == obstacles[i] - 1) {
          continue;
        }
        d[j] = min(d[j], minCnt + 1);
      }
    }
    return *min_element(d.begin(), d.end());
  }
};

int main() {
  {
    vector<int> obstacles{0, 1, 2, 3, 0};
    int ans = 2;
    assert(Solution().minSideJumps(obstacles) == ans);
  }

  {
    vector<int> obstacles{0, 1, 1, 3, 3, 0};
    int ans = 0;
    assert(Solution().minSideJumps(obstacles) == ans);
  }

  {
    vector<int> obstacles{0, 2, 1, 0, 3, 0};
    int ans = 2;
    assert(Solution().minSideJumps(obstacles) == ans);
  }
}