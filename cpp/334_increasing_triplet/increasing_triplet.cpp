/*
 * @Date: 2022-01-12 00:48:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-12 01:04:25
 */

#include <cassert>
#include <climits>
#include <vector>

using namespace std;

class Solution {
 public:
  bool increasingTriplet(vector<int> nums) {
    int n = nums.size();
    if (n < 3) {
      return false;
    }
    int first = nums[0], second = INT_MAX;
    for (int i = 1; i < n; i++) {
      int num = nums[i];
      if (num > second) {
        return true;
      } else if (num > first) {
        second = num;
      } else {
        first = num;
      }
    }
    return false;
  }
};

int main() {
  {
    assert(Solution().increasingTriplet({1, 2, 3, 4, 5}) == true);
    assert(Solution().increasingTriplet({5, 4, 3, 2, 1}) == false);
    assert(Solution().increasingTriplet({2, 1, 5, 0, 4, 6}) == true);
  }
}