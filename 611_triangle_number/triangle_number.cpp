/*
 * @Date: 2021-08-04 14:51:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-04 15:02:35
 */

#include <algorithm>
#include <cassert>
#include <vector>
using namespace std;

class Solution {
 public:
  int triangleNumber(vector<int>& nums) {
    int n = nums.size();
    sort(nums.begin(), nums.end());
    int ans = 0;
    for (int i = 0; i < n; ++i) {
      int k = i;
      for (int j = i + 1; j < n; ++j) {
        while (k + 1 < n && nums[k + 1] < nums[i] + nums[j]) ++k;
        ans += max(k - j, 0);
      }
    }
    return ans;
  }
};

int main() {
  vector<int> nums{2, 2, 3, 4};
  assert(Solution{}.triangleNumber(nums) == 3);
  return 0;
}