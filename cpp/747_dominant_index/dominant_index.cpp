/*
 * @Date: 2022-01-13 01:33:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-13 01:57:14
 */

#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int dominantIndex(vector<int> nums) {
    if (nums.size() == 1) return 0;
    int first = INT_MIN;
    int second = INT_MIN;
    int index = -1;

    for (auto [it, i] = std::tuple{nums.cbegin(), 0}; it != nums.cend();
         it++, i++) {
      if (*it > first) {
        second = first;
        first = *it;
        index = i;
      } else if (*it > second) {
        second = *it;
      }
    }

    return first >= second * 2 ? index : -1;
  }
};

int main() {
  assert(Solution().dominantIndex({3, 6, 1, 0}) == 1);
  assert(Solution().dominantIndex({1, 2, 3, 4}) == -1);
  assert(Solution().dominantIndex({1}) == 0);
}