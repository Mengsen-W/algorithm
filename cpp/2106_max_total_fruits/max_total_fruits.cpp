/*
 * @Date: 2023-05-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-04
 * @FilePath: /algorithm/cpp/2106_max_total_fruits/max_total_fruits.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxTotalFruits(vector<vector<int>>& fruits, int startPos, int k) {
    int left = 0;
    int right = 0;
    int n = fruits.size();
    int sum = 0;
    int ans = 0;

    auto step = [&](int left, int right) -> int {
      if (fruits[right][0] <= startPos) {
        return startPos - fruits[left][0];
      } else if (fruits[left][0] >= startPos) {
        return fruits[right][0] - startPos;
      } else {
        return min(abs(startPos - fruits[right][0]), abs(startPos - fruits[left][0])) + fruits[right][0] -
               fruits[left][0];
      }
    };
    // 每次固定住窗口右边界
    while (right < n) {
      sum += fruits[right][1];
      // 移动左边界
      while (left <= right && step(left, right) > k) {
        sum -= fruits[left][1];
        left++;
      }
      ans = max(ans, sum);
      right++;
    }
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> fruits{{2, 8}, {6, 3}, {8, 6}};
    int startPos = 5;
    int k = 4;
    int ans = 9;
    assert(Solution().maxTotalFruits(fruits, startPos, k) == ans);
  }

  {
    vector<vector<int>> fruits{{0, 9}, {4, 1}, {5, 7}, {6, 2}, {7, 4}, {10, 9}};
    int startPos = 5;
    int k = 4;
    int ans = 14;
    assert(Solution().maxTotalFruits(fruits, startPos, k) == ans);
  }

  {
    vector<vector<int>> fruits{{0, 3}, {6, 4}, {8, 5}};
    int startPos = 3;
    int k = 2;
    int ans = 0;
    assert(Solution().maxTotalFruits(fruits, startPos, k) == ans);
  }
}
