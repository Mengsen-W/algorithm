/*
 * @Date: 2023-07-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-23
 * @FilePath: /algorithm/cpp/42_trap/trap.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int trap(vector<int>& height) {
    int ans = 0;
    int left = 0, right = height.size() - 1;
    int leftMax = 0, rightMax = 0;
    while (left < right) {
      leftMax = max(leftMax, height[left]);
      rightMax = max(rightMax, height[right]);
      if (height[left] < height[right]) {
        ans += leftMax - height[left];
        ++left;
      } else {
        ans += rightMax - height[right];
        --right;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1}, 6},
      {{4, 2, 0, 3, 2, 5}, 9},
  };

  for (auto& [height, ans] : tests) {
    assert(Solution().trap(height) == ans);
  }
}
