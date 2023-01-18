/*
 * @Date: 2021-10-28 01:59:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-28 02:09:50
 */

#include <cassert>
#include <iostream>
#include <string>
#include <unordered_set>

using namespace std;

string countDigits(int n) {
  string cnt(10, 0);
  while (n) {
    ++cnt[n % 10];
    n /= 10;
  }
  return cnt;
}

unordered_set<string> powerOf2Digits = []() {
  unordered_set<string> res{};
  for (int n = 1; n <= 1e9; n <<= 1) {
    res.insert(countDigits(n));
  }
  return res;
}();

class Solution {
 public:
  bool reorderedPowerOf2(int n) { return powerOf2Digits.count(countDigits(n)); }
};

int main() {
  assert(Solution().reorderedPowerOf2(1) == true);
  assert(Solution().reorderedPowerOf2(10) == false);
  assert(Solution().reorderedPowerOf2(15) == false);
  assert(Solution().reorderedPowerOf2(16) == true);
  assert(Solution().reorderedPowerOf2(24) == false);
  assert(Solution().reorderedPowerOf2(46) == true);
  assert(Solution().reorderedPowerOf2(8208) == false);
  assert(Solution().reorderedPowerOf2(9474) == false);
  assert(Solution().reorderedPowerOf2(2147483647) == false);
}