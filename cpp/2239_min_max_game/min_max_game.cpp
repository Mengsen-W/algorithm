/*
 * @Date: 2023-01-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-15
 * @FilePath: /algorithm/2239_min_max_game/min_max_game.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minMaxGame(vector<int>& nums) {
    int n = nums.size();
    while (n != 1) {
      int m = n / 2;
      for (int i = 0; i < m; i++) {
        if (i % 2 == 0) {
          nums[i] = min(nums[2 * i], nums[2 * i + 1]);
        } else {
          nums[i] = max(nums[2 * i], nums[2 * i + 1]);
        }
      }
      n = m;
    }
    return nums[0];
  }
};

int main() {
  {
    vector<int> nums{1, 3, 5, 2, 4, 8, 2, 2};
    int ans = 1;
    assert(Solution().minMaxGame(nums) == ans);
  }

  {
    vector<int> nums{3};
    int ans = 3;
    assert(Solution().minMaxGame(nums) == ans);
  }
}