/*
 * @Date: 2023-06-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-04
 * @FilePath: /algorithm/cpp/2465_distinct_averages/distinct_averages.cpp
 */

#include <assert.h>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int distinctAverages(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    unordered_set<int> seen;
    for (int i = 0, j = nums.size() - 1; i < j; ++i, --j) {
      seen.insert(nums[i] + nums[j]);
    }
    return seen.size();
  }
};

int main() {
  {
    vector<int> nums{4, 1, 4, 0, 3, 5};
    int ans = 2;
    assert(Solution().distinctAverages(nums) == ans);
  }

  {
    vector<int> nums{100, 1};
    int ans = 1;
    assert(Solution().distinctAverages(nums) == ans);
  }
}