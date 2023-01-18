/*
 * @Date: 2022-08-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-09
 * @FilePath: /algorithm/1413_min_start_value/min_start_value.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minStartValue(vector<int> nums) {
    int m = *min_element(nums.begin(), nums.end());
    if (m >= 0) {
      return 1;
    }
    int left = 1, right = -m * nums.size() + 1;
    while (left < right) {
      int medium = (left + right) / 2;
      if (valid(medium, nums)) {
        right = medium;
      } else {
        left = medium + 1;
      }
    }
    return left;
  }

  bool valid(int startValue, vector<int>& nums) {
    for (int num : nums) {
      startValue += num;
      if (startValue <= 0) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  assert(Solution().minStartValue(vector<int>{-3, 2, -3, 4, 2}) == 5);
  assert(Solution().minStartValue(vector<int>{1, 2}) == 1);
  assert(Solution().minStartValue(vector<int>{1, -2, -3}) == 5);
}