/*
 * @Date: 2022-10-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-26
 * @FilePath: /algorithm/862_shortest_subarray/shortest_subarray.cpp
 */

#include <cassert>
#include <deque>
#include <vector>

using namespace std;

class Solution {
 public:
  int shortestSubarray(vector<int>& nums, int k) {
    int n = nums.size();
    vector<long> preSumArr(n + 1);
    for (int i = 0; i < n; i++) {
      preSumArr[i + 1] = preSumArr[i] + nums[i];
    }
    int res = n + 1;
    deque<int> qu;
    for (int i = 0; i <= n; i++) {
      long curSum = preSumArr[i];
      while (!qu.empty() && curSum - preSumArr[qu.front()] >= k) {
        res = min(res, i - qu.front());
        qu.pop_front();
      }
      while (!qu.empty() && preSumArr[qu.back()] >= curSum) {
        qu.pop_back();
      }
      qu.push_back(i);
    }
    return res < n + 1 ? res : -1;
  }
};

int main() {
  {
    vector<int> nums{1};
    int k = 1;
    int ans = 1;
    assert(Solution().shortestSubarray(nums, k) == ans);
  }

  {
    vector<int> nums{1, 2};
    int k = 4;
    int ans = -1;
    assert(Solution().shortestSubarray(nums, k) == ans);
  }

  {
    vector<int> nums{2, -1, 2};
    int k = 3;
    int ans = 3;
    assert(Solution().shortestSubarray(nums, k) == ans);
  }
}