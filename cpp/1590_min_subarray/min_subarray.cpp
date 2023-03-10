/*
 * @Date: 2023-03-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-10
 * @FilePath: /algorithm/cpp/1590_min_subarray/min_subarray.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int minSubarray(vector<int>& nums, int p) {
    int x = 0;
    for (auto num : nums) {
      x = (x + num) % p;
    }
    if (x == 0) {
      return 0;
    }
    unordered_map<int, int> index;
    int y = 0, res = nums.size();
    for (int i = 0; i < nums.size(); i++) {
      index[y] = i;  // f[i] mod p = y，因此哈希表记录 y 对应的下标为 i
      y = (y + nums[i]) % p;
      if (index.count((y - x + p) % p) > 0) {
        res = min(res, i - index[(y - x + p) % p] + 1);
      }
    }
    return res == nums.size() ? -1 : res;
  }
};

int main() {
  {
    vector<int> nums{3, 1, 4, 2};
    int p = 6;
    int ans = 1;
    assert(Solution().minSubarray(nums, p) == ans);
  }

  {
    vector<int> nums{6, 3, 5, 2};
    int p = 9;
    int ans = 2;
    assert(Solution().minSubarray(nums, p) == ans);
  }

  {
    vector<int> nums{1, 2, 3};
    int p = 3;
    int ans = 0;
    assert(Solution().minSubarray(nums, p) == ans);
  }

  {
    vector<int> nums{1, 2, 3};
    int p = 7;
    int ans = -1;
    assert(Solution().minSubarray(nums, p) == ans);
  }

  {
    vector<int> nums{1000000000, 1000000000, 1000000000};
    int p = 3;
    int ans = 0;
    assert(Solution().minSubarray(nums, p) == ans);
  }
}