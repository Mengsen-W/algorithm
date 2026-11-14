#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumSumSubarray(vector<int>& nums, int l, int r) {
    const int n = nums.size();
    int minSum = INT_MAX;

    int initial = 0;
    for (int i = 0; i < l - 1; ++i) {
      initial += nums[i];
    }

    for (int length = l; length <= r; ++length) {
      initial += nums[length - 1];
      int sum = initial;

      if (sum > 0 && sum < minSum) {
        minSum = sum;
      }

      for (int hi = length; hi < n; ++hi) {
        sum += nums[hi] - nums[hi - length];
        if (sum > 0 && sum < minSum) {
          minSum = sum;
        }
      }
    }

    return minSum == INT_MAX ? -1 : minSum;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, int>> tests{
      {{3, -2, 1, 4}, 2, 3, 1},
      {{-2, 2, -3, 1}, 2, 3, -1},
      {{1, 2, 3, 4}, 2, 4, 3},
  };

  for (auto [nums, l, r, expected] : tests) {
    assert(Solution().minimumSumSubarray(nums, l, r) == expected);
  }
}