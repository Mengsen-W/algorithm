/*
 * @Date: 2023-12-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-09
 * @FilePath: /algorithm/cpp/2048_next_beautiful_number/next_beautiful_number.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isBalance(int x) {
    vector<int> count(10);
    while (x > 0) {
      count[x % 10]++;
      x /= 10;
    }
    for (int d = 0; d < 10; ++d) {
      if (count[d] > 0 && count[d] != d) {
        return false;
      }
    }
    return true;
  }

  int nextBeautifulNumber(int n) {
    for (int i = n + 1; i <= 1224444; ++i) {
      if (isBalance(i)) {
        return i;
      }
    }
    return -1;
  }
};

int main() {
  vector<tuple<int, int>> tests{{1, 22}, {1000, 1333}, {3000, 3133}, {122645, 123233}};

  for (auto &[x, ans] : tests) {
    assert(Solution().nextBeautifulNumber(x) == ans);
  }
}