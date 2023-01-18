/*
 * @Date: 2022-05-21 21:41:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-21 21:44:01
 * @FilePath: /algorithm/961_repeated_n_times/repeated_n_times.cpp
 */

#include <cassert>
#include <random>
#include <vector>

using namespace std;

class Solution {
 public:
  int repeatedNTimes(vector<int> nums) {
    int n = nums.size();
    mt19937 gen{random_device{}()};
    uniform_int_distribution<int> dis(0, n - 1);

    while (true) {
      int x = dis(gen), y = dis(gen);
      if (x != y && nums[x] == nums[y]) {
        return nums[x];
      }
    }
  }
};

int main() {
  assert(Solution().repeatedNTimes({1, 2, 3, 3}) == 3);
  assert(Solution().repeatedNTimes({2, 1, 2, 5, 3, 2}) == 2);
  assert(Solution().repeatedNTimes({5, 1, 5, 2, 5, 3, 5, 4}) == 5);
}