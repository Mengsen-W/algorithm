/*
 * @Date: 2021-10-20 12:16:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-20 12:19:43
 */

#include <cassert>
#include <functional>
#include <vector>

using namespace std;

class Solution {
 public:
  int minMoves(vector<int> nums) {
    int minNum = *min_element(nums.begin(), nums.end());
    int res = 0;
    for (int num : nums) res += num - minNum;

    return res;
  }
};

int main() {
  assert(Solution().minMoves(vector<int>{1, 2, 3}) == 3);
  assert(Solution().minMoves(vector<int>{1, 1, 1}) == 0);
}