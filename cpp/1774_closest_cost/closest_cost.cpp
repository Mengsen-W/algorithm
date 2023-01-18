/*
 * @Date: 2022-12-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-04
 * @FilePath: /algorithm/1774_closest_cost/closest_cost.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int closestCost(vector<int>& baseCosts, vector<int>& toppingCosts, int target) {
    int x = *min_element(baseCosts.begin(), baseCosts.end());
    if (x >= target) {
      return x;
    }
    vector<bool> can(target + 1, false);
    int res = 2 * target - x;
    for (auto& b : baseCosts) {
      if (b <= target) {
        can[b] = true;
      } else {
        res = min(res, b);
      }
    }
    for (auto& t : toppingCosts) {
      for (int count = 0; count < 2; ++count) {
        for (int i = target; i; --i) {
          if (can[i] && i + t > target) {
            res = min(res, i + t);
          }
          if (i - t > 0) {
            can[i] = can[i] || can[i - t];
          }
        }
      }
    }
    for (int i = 0; i <= res - target; ++i) {
      if (can[target - i]) {
        return target - i;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<int> baseCosts{1, 7};
    vector<int> toppingCosts{3, 4};
    int target = 10;
    int ans = 10;
    assert(Solution().closestCost(baseCosts, toppingCosts, target) == ans);
  }

  {
    vector<int> baseCosts{2, 3};
    vector<int> toppingCosts{4, 5, 100};
    int target = 18;
    int ans = 17;
    assert(Solution().closestCost(baseCosts, toppingCosts, target) == ans);
  }

  {
    vector<int> baseCosts{3, 10};
    vector<int> toppingCosts{2, 5};
    int target = 9;
    int ans = 8;
    assert(Solution().closestCost(baseCosts, toppingCosts, target) == ans);
  }

  {
    vector<int> baseCosts{ 10};
    vector<int> toppingCosts{1};
    int target = 1;
    int ans = 10;
    assert(Solution().closestCost(baseCosts, toppingCosts, target) == ans);
  }
}
