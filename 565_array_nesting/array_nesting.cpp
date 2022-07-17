/*
 * @Date: 2022-07-17
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-17
 * @FilePath: /algorithm/565_array_nesting/array_nesting.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int arrayNesting(vector<int> nums) {
    int ans = 0, n = nums.size();
    for (int i = 0; i < n; ++i) {
      int cnt = 0;
      while (nums[i] < n) {
        int num = nums[i];
        nums[i] = n;
        i = num;
        ++cnt;
      }
      ans = max(ans, cnt);
    }
    return ans;
  }
};

int main() { assert(Solution().arrayNesting({5, 4, 0, 3, 1, 6, 2}) == 4); }