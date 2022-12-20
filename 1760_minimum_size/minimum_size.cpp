/*
 * @Date: 2022-12-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-20
 * @FilePath: /algorithm/1760_minimum_size/minimum_size.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumSize(vector<int>& nums, int maxOperations) {
    int left = 1, right = *max_element(nums.begin(), nums.end());
    int ans = 0;
    while (left <= right) {
      int y = (left + right) / 2;
      long long ops = 0;
      for (int x : nums) {
        ops += (x - 1) / y;
      }
      if (ops <= maxOperations) {
        ans = y;
        right = y - 1;
      } else {
        left = y + 1;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{9};
    int maxOperations = 2;
    int ans = 3;
    assert(Solution().minimumSize(nums, maxOperations) == ans);
  }

  {
    vector<int> nums{2, 4, 8, 2};
    int maxOperations = 4;
    int ans = 2;
    assert(Solution().minimumSize(nums, maxOperations) == ans);
  }

  {
    vector<int> nums{7, 17};
    int maxOperations = 2;
    int ans = 7;
    assert(Solution().minimumSize(nums, maxOperations) == ans);
  }
}