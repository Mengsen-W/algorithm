/*
 * @Date: 2023-03-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-26
 * @FilePath: /algorithm/cpp/2395_find_subarrays/find_subarrays.cpp
 */

#include <cassert>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  bool findSubarrays(vector<int>& nums) {
    int n = nums.size();
    unordered_set<int> seen;
    for (int i = 0; i < n - 1; ++i) {
      int sum = nums[i] + nums[i + 1];
      if (seen.count(sum)) {
        return true;
      }
      seen.insert(sum);
    }
    return false;
  }
};

int main() {
  {
    vector<int> nums{4, 2, 4};
    bool ans = true;
    assert(Solution().findSubarrays(nums) == ans);
  }

  {
    vector<int> nums{1, 2, 3, 4, 5};
    bool ans = false;
    assert(Solution().findSubarrays(nums) == ans);
  }

  {
    vector<int> nums{0, 0, 0};
    bool ans = true;
    assert(Solution().findSubarrays(nums) == ans);
  }
}