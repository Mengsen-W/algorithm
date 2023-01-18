/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-16 22:22:25
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-16 22:37:07
 */

#include <algorithm>
#include <cassert>
#include <functional>
#include <vector>

using namespace std;

int array_pair_sum(vector<int>& nums) {
  std::sort(nums.begin(), nums.end(), std::less<int>());
  int ans = 0;
  for (int i = 0; i < nums.size(); i += 2) {
    ans += nums[i];
  }
  return ans;
}

int main(void) {
  vector<int> nums{};
  nums = {1, 4, 3, 2};
  assert(array_pair_sum(nums) == 4);
  nums = {6, 2, 6, 5, 1, 2};
  assert(array_pair_sum(nums) == 9);
}