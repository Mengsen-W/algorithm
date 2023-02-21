/*
 * @Date: 2023-02-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-21
 * @FilePath: /algorithm/cpp/1326_min_taps/min_taps.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int minTaps(int n, vector<int>& ranges) {
    vector<int> rightMost(n + 1);
    iota(rightMost.begin(), rightMost.end(), 0);
    for (int i = 0; i <= n; i++) {
      int start = max(0, i - ranges[i]);
      int end = min(n, i + ranges[i]);
      rightMost[start] = max(rightMost[start], end);
    }
    int last = 0, ret = 0, pre = 0;
    for (int i = 0; i < n; i++) {
      last = max(last, rightMost[i]);
      if (i == last) {
        return -1;
      }
      if (i == pre) {
        ret++;
        pre = last;
      }
    }
    return ret;
  }
};

int main() {
  {
    int n = 5;
    vector<int> ranges{3, 4, 1, 1, 0, 0};
    int ans = 1;
    assert(Solution().minTaps(n, ranges) == ans);
  }

  {
    int n = 3;
    vector<int> ranges{0,0,0,0};
    int ans = -1;
    assert(Solution().minTaps(n, ranges) == ans);
  }
}