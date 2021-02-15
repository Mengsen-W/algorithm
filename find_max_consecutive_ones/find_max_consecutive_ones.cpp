/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-15 10:57:53
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-15 11:12:00
 */

#include <cassert>
#include <vector>

using namespace std;

int find_max_consecutive_ones(vector<int> &nums) {
  int count = 0, max_count = 0;

  for (int num : nums) {
    if (num == 1) {
      ++count;
    } else {
      max_count = max(max_count, count);
      count = 0;
    }
  }
  return max(max_count, count);
}

int main() {
  vector<int> nums = {1, 0, 1, 1, 0, 1};
  assert(find_max_consecutive_ones(nums) == 2);
  return 0;
}