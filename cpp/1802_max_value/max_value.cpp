/*
 * @Date: 2023-01-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-04
 * @FilePath: /algorithm/1802_max_value/max_value.cpp
 */

#include <cassert>
#include <cmath>

using namespace std;

class Solution {
 public:
  int maxValue(int n, int index, int maxSum) {
    double left = index;
    double right = n - index - 1;
    if (left > right) {
      double temp = left;
      left = right;
      right = temp;
    }

    double upper = ((double)(left + 1) * (left + 1) - 3 * (left + 1)) / 2 + left + 1 + (left + 1) +
                   ((left + 1) * (left + 1) - 3 * (left + 1)) / 2 + right + 1;
    if (upper >= maxSum) {
      double a = 1;
      double b = -2;
      double c = left + right + 2 - maxSum;
      return (int)floor((-b + sqrt(b * b - 4 * a * c)) / (2 * a));
    }

    upper = ((double)2 * (right + 1) - left - 1) * left / 2 + (right + 1) +
            ((right + 1) * (right + 1) - 3 * (right + 1)) / 2 + right + 1;
    if (upper >= maxSum) {
      double a = 1.0 / 2;
      double b = left + 1 - 3.0 / 2;
      double c = right + 1 + (-left - 1) * left / 2 - maxSum;
      return (int)floor((-b + sqrt(b * b - 4 * a * c)) / (2 * a));
    } else {
      double a = left + right + 1;
      double b = (-left * left - left - right * right - right) / 2 - maxSum;
      return (int)floor(-b / a);
    }
  }
};

int main() {
  {
    int n = 4;
    int index = 2;
    int maxSum = 6;
    int ans = 2;
    assert(Solution().maxValue(n, index, maxSum) == ans);
  }

  {
    int n = 6;
    int index = 1;
    int maxSum = 10;
    int ans = 3;
    assert(Solution().maxValue(n, index, maxSum) == ans);
  }
}