#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minPairSum(vector<int>& nums) {
    int n = nums.size();
    int res = 0;
    sort(nums.begin(), nums.end());
    for (int i = 0; i < n / 2; ++i) {
      res = max(res, nums[i] + nums[n - 1 - i]);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 5, 2, 3}, 7},
      {{3, 5, 4, 2, 4, 6}, 8},
  };

  for (auto [nums, expect] : tests) {
    assert(Solution().minPairSum(nums) == expect);
  }
}