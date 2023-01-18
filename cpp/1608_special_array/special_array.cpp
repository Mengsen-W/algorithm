/*
 * @Date: 2022-09-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-12
 * @FilePath: /algorithm/1608_special_array/special_array.cpp
 */

#include <assert.h>

#include <vector>

using namespace std;

class Solution {
 public:
  int specialArray(vector<int>& nums) {
    sort(nums.begin(), nums.end(), greater<int>());
    int n = nums.size();
    for (int i = 1; i <= n; ++i) {
      if (nums[i - 1] >= i && (i == n || nums[i] < i)) {
        return i;
      }
    }
    return -1;
  }
};

int main() {
  {
    vector<int> nums{3, 5};
    int ans = 2;
    assert(Solution().specialArray(nums) == ans);
  }

  {
    vector<int> nums{0, 0};
    int ans = -1;
    assert(Solution().specialArray(nums) == ans);
  }

  {
    vector<int> nums{0, 4, 3, 0, 4};
    int ans = 3;
    assert(Solution().specialArray(nums) == ans);
  }

  {
    vector<int> nums{3, 6, 7, 7, 0};
    int ans = -1;
    assert(Solution().specialArray(nums) == ans);
  }
}
