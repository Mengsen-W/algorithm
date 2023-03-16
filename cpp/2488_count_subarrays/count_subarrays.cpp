/*
 * @Date: 2023-03-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-16
 * @FilePath: /algorithm/cpp/2488_count_subarrays/count_subarrays.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  inline int sign(int num) {
    if (num == 0) {
      return 0;
    }
    return num > 0 ? 1 : -1;
  }

  int countSubarrays(vector<int>& nums, int k) {
    int n = nums.size();
    int kIndex = -1;
    for (int i = 0; i < n; i++) {
      if (nums[i] == k) {
        kIndex = i;
        break;
      }
    }
    int ans = 0;
    unordered_map<int, int> counts;
    counts[0] = 1;
    int sum = 0;
    for (int i = 0; i < n; i++) {
      sum += sign(nums[i] - k);
      if (i < kIndex) {
        counts[sum]++;
      } else {
        int prev0 = counts[sum];
        int prev1 = counts[sum - 1];
        ans += prev0 + prev1;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{3, 2, 1, 4, 5};
    int k = 4;
    int ans = 3;
    assert(Solution().countSubarrays(nums, k) == ans);
  }

  {
    vector<int> nums{2, 3, 1};
    int k = 3;
    int ans = 1;
    assert(Solution().countSubarrays(nums, k) == ans);
  }
}