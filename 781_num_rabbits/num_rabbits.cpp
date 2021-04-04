/*
 * @Date: 2021-04-04 19:46:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-04 19:49:02
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

int num_rabbits(vector<int> &answers) {
  unordered_map<int, int> count;
  for (int y : answers) {
    ++count[y];
  }
  int ans = 0;
  for (auto &[y, x] : count) {
    ans += (x + y) / (y + 1) * (y + 1);
  }
  return ans;
}

int main() {
  vector<int> nums{1, 1, 2};
  assert(5 == num_rabbits(nums));
  nums = {10, 10, 10};
  assert(11 == num_rabbits(nums));
  nums = {};
  assert(0 == num_rabbits(nums));
  return 0;
}