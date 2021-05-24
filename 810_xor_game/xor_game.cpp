/*
 * @Date: 2021-05-24 10:20:03
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-24 10:25:38
 */

#include <cassert>
#include <vector>
using namespace std;

bool xorGame(const vector<int>& nums) {
  if (nums.size() % 2 == 0) {
    return true;
  }

  int xorsum = 0;

  for (int num : nums) {
    xorsum ^= num;
  }

  return xorsum == 0;
}

int main() {
  assert(!xorGame(vector<int>{1, 1, 2}));
  return 0;
}