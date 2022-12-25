/*
 * @Date: 2022-12-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-25
 * @FilePath: /algorithm/1739_minimum_boxes/minimum_boxes.cpp
 */

#include <cassert>
#include <cmath>

using namespace std;

class Solution {
 public:
  int g(int x) { return (long long)x * (x + 1) * (x + 2) / 6; }

  int minimumBoxes(int n) {
    int i = pow(6.0 * n, 1.0 / 3);
    if (g(i) < n) {
      i++;
    }
    n -= g(i - 1);
    int j = ceil(1.0 * (sqrt((long long)8 * n + 1) - 1) / 2);
    return (i - 1) * i / 2 + j;
  }
};

int main() {
  {
    int n = 3;
    int ans = 3;
    assert(Solution().minimumBoxes(n) == ans);
  }

  {
    int n = 4;
    int ans = 3;
    assert(Solution().minimumBoxes(n) == ans);
  }

  {
    int n = 10;
    int ans = 6;
    assert(Solution().minimumBoxes(n) == ans);
  }
}