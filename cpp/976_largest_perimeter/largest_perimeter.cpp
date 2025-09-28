#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int largestPerimeter(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    for (int i = (int)nums.size() - 1; i >= 2; --i) {
      if (nums[i - 2] + nums[i - 1] > nums[i]) {
        return nums[i - 2] + nums[i - 1] + nums[i];
      }
    }
    return 0;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 1, 2}, 5},
      {{1, 2, 1, 10}, 0},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().largestPerimeter(nums) == ans);
  }
}
