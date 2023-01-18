/*
 * @Date: 2022-04-25 09:28:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-25 09:32:53
 * @FilePath: /algorithm/398_pick_random_index/pick_random_index.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
  vector<int> &nums;

 public:
  Solution(vector<int> &nums) : nums(nums) {}

  int pick(int target) {
    int ans;
    for (int i = 0, cnt = 0; i < nums.size(); ++i) {
      if (nums[i] == target) {
        ++cnt;  // 第 cnt 次遇到 target
        if (rand() % cnt == 0) {
          ans = i;
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<int> nums = {1, 2, 3, 3, 3};
  Solution s{nums};
  assert(s.pick(1) == 0);
  assert(s.pick(2) == 1);
  assert(s.pick(3) == 2 || s.pick(3) == 3 || s.pick(3) == 4);
}