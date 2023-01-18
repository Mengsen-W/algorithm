/*
 * @Date: 2022-12-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-17
 * @FilePath: /algorithm/1764_can_choose/can_choose.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int find(vector<int>& nums, int k, vector<int>& g) {
    int m = g.size(), n = nums.size();
    if (k + g.size() > nums.size()) {
      return -1;
    }
    vector<int> pi(m);
    for (int i = 1, j = 0; i < m; i++) {
      while (j > 0 && g[i] != g[j]) {
        j = pi[j - 1];
      }
      if (g[i] == g[j]) {
        j++;
      }
      pi[i] = j;
    }
    for (int i = k, j = 0; i < n; i++) {
      while (j > 0 && nums[i] != g[j]) {
        j = pi[j - 1];
      }
      if (nums[i] == g[j]) {
        j++;
      }
      if (j == m) {
        return i - m + 1;
      }
    }
    return -1;
  }

  bool canChoose(vector<vector<int>>& groups, vector<int>& nums) {
    int k = 0;
    for (int i = 0; i < groups.size(); i++) {
      k = find(nums, k, groups[i]);
      if (k == -1) {
        return false;
      }
      k += groups[i].size();
    }
    return true;
  }
};

int main() {
  {
    vector<vector<int>> groups{{1, -1, -1}, {3, -2, 0}};
    vector<int> nums{1, -1, 0, 1, -1, -1, 3, -2, 0};
    assert(Solution().canChoose(groups, nums));
  }

  {
    vector<vector<int>> groups{{10, -2}, {1, 2, 3, 4}};
    vector<int> nums{1, 2, 3, 4, 10, -2};
    assert(!Solution().canChoose(groups, nums));
  }

  {
    vector<vector<int>> groups{{1, 2, 3}, {3, 4}};
    vector<int> nums{7, 7, 1, 2, 3, 4, 7, 7};
    assert(!Solution().canChoose(groups, nums));
  }
}