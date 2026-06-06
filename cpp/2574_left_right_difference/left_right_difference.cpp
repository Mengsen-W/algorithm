#include <cassert>
#include <cstdlib>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> leftRightDifference(vector<int>& nums) {
    int n = nums.size();
    vector<int> ans(n);

    int leftSum = 0;
    for (int i = 0; i < n; ++i) {
      ans[i] = leftSum;
      leftSum += nums[i];
    }

    int rightSum = 0;
    for (int i = n - 1; i >= 0; --i) {
      ans[i] = abs(ans[i] - rightSum);
      rightSum += nums[i];
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{10, 4, 8, 3}, {15, 1, 11, 22}},
      {{1}, {0}},
  };

  for (auto& [nums, expected] : tests) {
    assert(Solution().leftRightDifference(nums) == expected);
  }
}