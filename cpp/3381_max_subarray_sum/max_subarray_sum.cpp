#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maxSubarraySum(vector<int>& nums, int k) {
    int n = nums.size();
    long long prefixSum = 0, maxSum = LONG_LONG_MIN;
    vector<long long> kSum(k, LONG_LONG_MAX / 2);
    kSum[k - 1] = 0;
    for (int i = 0; i < n; i++) {
      prefixSum += nums[i];
      maxSum = max(maxSum, prefixSum - kSum[i % k]);
      kSum[i % k] = min(kSum[i % k], prefixSum);
    }
    return maxSum;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{1, 2}, 1, 3},
      {{-1, -2, -3, -4, -5}, 4, -10},
      {{-5, 1, 2, -3, 4}, 2, 4},
  };

  for (auto& [nums, k, expect] : tests) {
    assert(Solution().maxSubarraySum(nums, k) == expect);
  }
  return 0;
}