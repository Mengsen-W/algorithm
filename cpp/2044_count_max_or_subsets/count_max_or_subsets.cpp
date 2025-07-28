#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countMaxOrSubsets(vector<int>& nums_) {
    this->nums = nums_;
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
  vector<tuple<vector<int>, int>> testCases{
      {{3, 1}, 2},
      {{2, 2, 2}, 7},
      {{3, 2, 1, 5}, 6},
  };

  for (auto& [nums, expected] : testCases) {
    assert(Solution().countMaxOrSubsets(nums) == expected);
  }
}