/*
 * @Date: 2023-03-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-31
 * @FilePath: /algorithm/cpp/2367_arithmetic_triplets/arithmetic_triplets.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int arithmeticTriplets(vector<int>& nums, int diff) {
    int ans = 0;
    int n = nums.size();
    for (int i = 0, j = 1, k = 2; i < n - 2 && j < n - 1 && k < n; i++) {
      j = max(j, i + 1);
      while (j < n - 1 && nums[j] - nums[i] < diff) {
        j++;
      }
      if (j >= n - 1 || nums[j] - nums[i] > diff) {
        continue;
      }
      k = max(k, j + 1);
      while (k < n && nums[k] - nums[j] < diff) {
        k++;
      }
      if (k < n && nums[k] - nums[j] == diff) {
        ans++;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{0, 1, 4, 6, 7, 10};
    int diff = 3;
    int ans = 2;
    assert(Solution().arithmeticTriplets(nums, diff) == ans);
  }

  {
    vector<int> nums{0, 1, 4, 6, 7, 10};
    int diff = 3;
    int ans = 2;
    assert(Solution().arithmeticTriplets(nums, diff) == ans);
  }

  {
    vector<int> nums{4, 5, 6, 7, 8, 9};
    int diff = 2;
    int ans = 2;
    assert(Solution().arithmeticTriplets(nums, diff) == ans);
  }
}