#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxAdjacentDistance(vector<int>& nums) {
    int n = nums.size();
    int res = abs(nums[0] - nums[n - 1]);
    for (int i = 0; i < n - 1; ++i) {
      res = max(res, abs(nums[i] - nums[i + 1]));
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 4}, 3},
      {{-5, -10, -5}, 5},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().maxAdjacentDistance(nums) == ans);
  }
}