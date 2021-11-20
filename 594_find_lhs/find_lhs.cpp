/*
 * @Date: 2021-11-20 00:40:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-20 00:44:43
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int findLHS(vector<int> nums) {
    unordered_map<int, int> cnt;
    int res = 0;
    for (int num : nums) cnt[num]++;

    for (auto [key, val] : cnt) {
      if (cnt.count(key + 1)) res = max(res, val + cnt[key + 1]);
    }
    return res;
  }
};

int main() {
  assert(Solution().findLHS({1, 3, 2, 2, 5, 2, 3, 7}) == 5);
  assert(Solution().findLHS({1, 2, 3, 4}) == 2);
  assert(Solution().findLHS({1, 1, 1, 1}) == 0);
}