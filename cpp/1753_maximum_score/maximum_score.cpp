/*
 * @Date: 2022-12-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-21
 * @FilePath: /algorithm/1753_maximum_score/maximum_score.cpp
 */

#include <algorithm>
#include <cassert>

using namespace std;

class Solution {
 public:
  int maximumScore(int a, int b, int c) {
    int sum = a + b + c;
    int maxVal = max({a, b, c});
    if (sum - maxVal < maxVal) {
      return sum - maxVal;
    } else {
      return sum / 2;
    }
  }
};

int main() {
  {
    int a = 2;
    int b = 4;
    int c = 6;
    int ans = 6;
    assert(Solution().maximumScore(a, b, c) == ans);
  }

  {
    int a = 4;
    int b = 4;
    int c = 6;
    int ans = 7;
    assert(Solution().maximumScore(a, b, c) == ans);
  }

  {
    int a = 1;
    int b = 8;
    int c = 8;
    int ans = 8;
    assert(Solution().maximumScore(a, b, c) == ans);
  }
}