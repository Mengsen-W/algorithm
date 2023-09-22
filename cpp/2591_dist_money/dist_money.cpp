/*
 * @Date: 2023-09-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-22
 * @FilePath: /algorithm/cpp/2591_dist_money/dist_money.cpp
 */

#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int distMoney(int money, int children) {
    if (money < children) {
      return -1;
    }
    money -= children;
    int cnt = min(money / 7, children);
    money -= cnt * 7;
    children -= cnt;
    if ((children == 0 && money > 0) || (children == 1 && money == 3)) {
      cnt--;
    }
    return cnt;
  }
};

int main() {
  vector<tuple<int, int, int>> tests{
      {20, 3, 1},
      {16, 2, 2},
  };

  for (auto &[money, children, expect] : tests) {
    assert(Solution().distMoney(money, children) == expect);
  }
}
