#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findValueOfPartition(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    int res = INT_MAX;
    for (int i = 1; i < nums.size(); i++) {
      res = min(res, nums[i] - nums[i - 1]);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 3, 2, 4}, 1},
      {{100, 1, 10}, 9},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().findValueOfPartition(nums) == ans);
  }
}