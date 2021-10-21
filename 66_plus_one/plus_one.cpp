/*
 * @Date: 2021-10-21 01:13:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-21 01:28:54
 */

#include <cassert>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> plusOne(vector<int>& digits) {
    int n = digits.size();
    // 从最低位数开始加1，如果进位则置0，否则
    while (n && ++digits[--n] == 10) digits[n] = 0;
    // 判断最高位
    if (digits[0] == 0) digits.insert(begin(digits), 1);
    return digits;
  }
};

int main() {
  {
    vector<int> digits{1, 2, 3};
    Solution().plusOne(digits);
    assert(digits == move(vector<int>{1, 2, 4}));
  }
  {
    vector<int> digits{1, 2, 9};
    Solution().plusOne(digits);
    assert(digits == move(vector<int>{1, 3, 0}));
  }
}