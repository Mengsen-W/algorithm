
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maxStrength(vector<int>& nums) {
    int negativeCount = 0, zeroCount = 0, positiveCount = 0;
    long long prod = 1;
    int maxNegative = INT_MIN;
    for (int num : nums) {
      if (num < 0) {
        negativeCount++;
        prod *= num;
        maxNegative = max(maxNegative, num);
      } else if (num == 0) {
        zeroCount++;
      } else {
        prod *= num;
        positiveCount++;
      }
    }

    if (negativeCount == 1 && zeroCount == 0 && positiveCount == 0) {
      return nums[0];
    }
    if (negativeCount <= 1 && positiveCount == 0) {
      return 0;
    }
    if (prod < 0) {
      return prod / maxNegative;
    } else {
      return prod;
    }
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{3, -1, -5, 2, 5, -9}, 1350},
      {{-4, -5, -4}, 20},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().maxStrength(nums) == ans);
  }
}