/*
 * @Date: 2023-01-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-07
 * @FilePath: /algorithm/1658_min_operations/min_operations.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int>& nums, int x) {
    int n = nums.size();
    int sum = accumulate(nums.begin(), nums.end(), 0);

    if (sum < x) {
      return -1;
    }

    int right = 0;
    int lsum = 0, rsum = sum;
    int ans = n + 1;

    for (int left = -1; left < n; ++left) {
      if (left != -1) {
        lsum += nums[left];
      }
      while (right < n && lsum + rsum > x) {
        rsum -= nums[right];
        ++right;
      }
      if (lsum + rsum == x) {
        ans = min(ans, (left + 1) + (n - right));
      }
    }

    return ans > n ? -1 : ans;
  }
};

int main() {
  {
    vector<int> nums{1, 1, 4, 2, 3};
    int x = 5;
    int ans = 2;
    assert(Solution().minOperations(nums, x) == ans);
  }

  {
    vector<int> nums{5, 6, 7, 8, 9};
    int x = 4;
    int ans = -1;
    assert(Solution().minOperations(nums, x) == ans);
  }

  {
    vector<int> nums{3, 2, 20, 1, 1, 3};
    int x = 10;
    int ans = 5;
    assert(Solution().minOperations(nums, x) == ans);
  }
}