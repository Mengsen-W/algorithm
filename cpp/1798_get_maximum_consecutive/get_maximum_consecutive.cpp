/*
 * @Date: 2023-02-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-04
 * @FilePath: /algorithm/cpp/1798_get_maximum_consecutive/get_maximum_consecutive.cpp
 */

#include <vector>
#include <algorithm>
#include <cassert>

using namespace std;

class Solution {
 public:
  int getMaximumConsecutive(vector<int>& coins) {
    int res = 1;
    sort(coins.begin(), coins.end());
    for (auto& i : coins) {
      if (i > res) {
        break;
      }
      res += i;
    }
    return res;
  }
};

int main() {
  {
    vector<int> coins {1,3};
    int ans = 2;
    assert(Solution().getMaximumConsecutive(coins) == ans);
  }

  {
    vector<int> coins {1,1,1,4};
    int ans = 8;
    assert(Solution().getMaximumConsecutive(coins) == ans);
  }


  {
    vector<int> coins {1,4,10,3,1};
    int ans = 20;
    assert(Solution().getMaximumConsecutive(coins) == ans);
  }
}