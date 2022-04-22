/*
 * @Date: 2022-04-22 09:19:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-22 09:23:44
 * @FilePath: /algorithm/396_max_rotate_function/max_rotate_function.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxRotateFunction(vector<int> nums) {
    int f = 0, n = nums.size();
    int numSum = accumulate(nums.begin(), nums.end(), 0);
    for (int i = 0; i < n; i++) {
      f += i * nums[i];
    }
    int res = f;
    for (int i = n - 1; i > 0; i--) {
      f += numSum - n * nums[i];
      res = max(res, f);
    }
    return res;
  }
};

int main() {
  assert(Solution().maxRotateFunction(vector<int>{4, 3, 2, 6}) == 26);
  assert(Solution().maxRotateFunction(vector<int>{100}) == 0);
}