/*
 * @Date: 2023-06-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-13
 * @FilePath: /algorithm/cpp/2475_unequal_triplets/unequal_triplets.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int unequalTriplets(vector<int>& nums) {
    unordered_map<int, int> count;
    for (auto x : nums) {
      count[x]++;
    }
    int res = 0, n = nums.size(), t = 0;
    for (auto [_, v] : count) {
      res += t * v * (n - t - v);
      t += v;
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums{4, 4, 2, 4, 3};
    int ans = 3;
    assert(Solution().unequalTriplets(nums) == ans);
  }

  {
    vector<int> nums{1, 1, 1, 1, 1};
    int ans = 0;
    assert(Solution().unequalTriplets(nums) == ans);
  }
}