/*
 * @Date: 2023-01-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-17
 * @FilePath: /algorithm/1814_count_nice_pairs/count_nice_pairs.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int countNicePairs(vector<int>& nums) {
    const int MOD = 1000000007;
    int res = 0;
    unordered_map<int, int> h;
    for (int i : nums) {
      int temp = i, j = 0;
      while (temp > 0) {
        j = j * 10 + temp % 10;
        temp /= 10;
      }
      res = (res + h[i - j]) % MOD;
      h[i - j]++;
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums{42, 11, 1, 97};
    int ans = 2;
    assert(Solution().countNicePairs(nums) == ans);
  }

  {
    vector<int> nums{13, 10, 35, 24, 76};
    int ans = 4;
    assert(Solution().countNicePairs(nums) == ans);
  }
}