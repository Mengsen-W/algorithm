/*
 * @Date: 2021-11-06 01:09:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-06 01:13:33
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int missingNumber(vector<int> nums) {
    int n = nums.size();
    int total = n * (n + 1) / 2;
    int arrSum = 0;
    for (int i = 0; i < n; i++) {
      arrSum += nums[i];
    }
    return total - arrSum;
  }
};

int main() {
  assert(Solution().missingNumber({3, 0, 1}) == 2);
  assert(Solution().missingNumber({9, 6, 4, 2, 3, 5, 7, 0, 1}) == 8);
  assert(Solution().missingNumber({0, 1}) == 2);
  assert(Solution().missingNumber({0}) == 1);
}