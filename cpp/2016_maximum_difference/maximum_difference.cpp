#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumDifference(vector<int> nums) {
    int n = nums.size();
    int ans = -1, premin = nums[0];
    for (int i = 1; i < n; ++i) {
      if (nums[i] > premin) {
        ans = max(ans, nums[i] - premin);
      } else {
        premin = nums[i];
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{7, 1, 5, 4}, 4},
      {{9, 4, 3, 2}, -1},
      {{1, 5, 2, 10}, 9},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().maximumDifference(nums) == ans);
  }
}