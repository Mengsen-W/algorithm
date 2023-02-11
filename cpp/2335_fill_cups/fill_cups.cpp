/*
 * @Date: 2023-02-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-11
 * @FilePath: /algorithm/cpp/2335_fill_cups/fill_cups.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int fillCups(vector<int>& amount) {
    sort(amount.begin(), amount.end());
    if (amount[2] > amount[1] + amount[0]) {
      return amount[2];
    }
    return (accumulate(amount.begin(), amount.end(), 0) + 1) / 2;
  }
};

int main() {
  {
    vector<int> amount{1, 4, 2};
    int ans = 4;
    assert(Solution().fillCups(amount) == ans);
  }

  {
    vector<int> amount{5, 4, 4};
    int ans = 7;
    assert(Solution().fillCups(amount) == ans);
  }

  {
    vector<int> amount{5, 0, 0};
    int ans = 5;
    assert(Solution().fillCups(amount) == ans);
  }
}