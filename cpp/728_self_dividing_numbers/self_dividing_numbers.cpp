/*
 * @Date: 2022-03-31 13:10:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-31 13:28:44
 * @FilePath: /algorithm/728_self_dividing_numbers/self_dividing_numbers.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isSelfDividing(int num) {
    int temp = num;
    while (temp > 0) {
      int digit = temp % 10;
      if (digit == 0 || num % digit != 0) {
        return false;
      }
      temp /= 10;
    }
    return true;
  }

  vector<int> selfDividingNumbers(int left, int right) {
    vector<int> ans;
    for (int i = left; i <= right; i++) {
      if (isSelfDividing(i)) {
        ans.emplace_back(i);
      }
    }
    return ans;
  }
};

int main() {
  assert((Solution().selfDividingNumbers(1, 22) ==
          vector<int>{1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22}));
  assert(
      (Solution().selfDividingNumbers(47, 85) == vector<int>{48, 55, 66, 77}));
}