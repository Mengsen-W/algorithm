/*
 * @Date: 2022-12-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-18
 * @FilePath: /algorithm/1703_min_moves/min_moves.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minMoves(vector<int>& nums, int k) {
    vector<int> g;
    vector<int> preSum(1, 0);
    for (int i = 0; i < nums.size(); i++) {
      if (nums[i] == 1) {
        g.emplace_back(i - g.size());
        preSum.emplace_back(preSum.back() + g.back());
      }
    }
    int m = g.size(), res = INT_MAX;
    for (int i = 0; i <= m - k; i++) {
      int mid = i + k / 2;
      res = min(res, (1 - k % 2) * g[mid] + (preSum[i + k] - preSum[mid + 1]) - (preSum[mid] - preSum[i]));
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums{1, 0, 0, 1, 0, 1};
    int k = 2;
    int ans = 1;
    assert(Solution().minMoves(nums, k) == ans);
  }

  {
    vector<int> nums{1, 0, 0, 0, 0, 0, 1, 1};
    int k = 3;
    int ans = 5;
    assert(Solution().minMoves(nums, k) == ans);
  }

  {
    vector<int> nums{1, 1, 0, 1};
    int k = 2;
    int ans = 0;
    assert(Solution().minMoves(nums, k) == ans);
  }
}