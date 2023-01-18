/*
 * @Date: 2022-02-06 02:18:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-06 02:27:13
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int sumOfUnique(vector<int> nums) {
    int ans = 0;
    unordered_map<int, int> state;
    for (int num : nums) {
      if (state[num] == 0) {
        ans += num;
        state[num] = 1;
      } else if (state[num] == 1) {
        ans -= num;
        state[num] = 2;
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().sumOfUnique({1, 2, 3, 2}) == 4);
  assert(Solution().sumOfUnique({1, 1, 1, 1, 1}) == 0);
  assert(Solution().sumOfUnique({1, 2, 3, 4, 5}) == 15);
}