#include <algorithm>
#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumDifference(vector<int> nums, int k) {
    int n = nums.size();
    sort(nums.begin(), nums.end());
    int ans = INT_MAX;
    for (int i = 0; i + k - 1 < n; ++i) {
      ans = min(ans, nums[i + k - 1] - nums[i]);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{90}, 1, 0},
      {{9, 4, 7, 1}, 2, 2},
  };

  for (auto [nums, k, expected] : tests) {
    assert(Solution().minimumDifference(nums, k) == expected);
  }
}