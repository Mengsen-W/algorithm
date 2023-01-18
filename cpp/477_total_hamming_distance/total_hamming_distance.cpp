/*
 * @Date: 2021-05-28 09:52:57
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-28 09:57:29
 */

#include <cassert>
#include <vector>
using namespace std;

int totalHammingDistance(vector<int> nums) {
  int ans = 0, n = nums.size();
  for (int i = 31; i >= 0; --i) {
    int count = 0;
    for (auto& a : nums) {
      if (a & (1 << i)) ++count;
    }
    ans += count * (n - count);
  }
  return ans;
}

int main() { assert(totalHammingDistance(vector<int>{4, 14, 2}) == 6); }