/*
 * @Date: 2022-03-15 00:28:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-15 00:49:31
 * @FilePath: /algorithm/2044_count_max_or_subsets/count_max_or_subsets.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int countMaxOrSubsets(vector<int>& nums) {
    this->nums = nums;
    this->maxOr = 0;
    this->cnt = 0;
    dfs(0, 0);
    return cnt;
  }

  void dfs(int pos, int orVal) {
    if (pos == nums.size()) {
      if (orVal > maxOr) {
        maxOr = orVal;
        cnt = 1;
      } else if (orVal == maxOr) {
        cnt++;
      }
      return;
    }
    dfs(pos + 1, orVal | nums[pos]);
    dfs(pos + 1, orVal);
  }

 private:
  vector<int> nums;
  int maxOr, cnt;
};

int main() {
  {
    vector<int> input{3, 1};
    int ans = 2;
    assert(Solution().countMaxOrSubsets(input) == ans);
  }

  {
    vector<int> input{2, 2, 2};
    int ans = 7;
    assert(Solution().countMaxOrSubsets(input) == ans);
  }

  {
    vector<int> input{3, 2, 1, 5};
    int ans = 6;
    assert(Solution().countMaxOrSubsets(input) == ans);
  }
}