/*
 * @Date: 2021-12-29 01:26:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-29 01:44:02
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int countQuadruplets(vector<int> nums) {
    int n = nums.size();
    int ans = 0;
    unordered_map<int, int> cnt;
    for (int b = n - 3; b >= 1; --b) {
      for (int d = b + 2; d < n; ++d) {
        ++cnt[nums[d] - nums[b + 1]];
      }
      for (int a = 0; a < b; ++a) {
        if (int sum = nums[a] + nums[b]; cnt.count(sum)) {
          ans += cnt[sum];
        }
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().countQuadruplets({1, 2, 3, 6}) == 1);
  assert(Solution().countQuadruplets({3, 3, 6, 4, 5}) == 0);
  assert(Solution().countQuadruplets({1,1,1,3,5}) == 4);
}