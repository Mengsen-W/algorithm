#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int maxLength(vector<int>& nums) {
    int res = 2;
    int prodVal = 1;
    int n = nums.size();
    int start = 0, end = 0;

    while (end < n) {
      while (gcd(prodVal, nums[end]) > 1) {
        prodVal /= nums[start];
        start++;
      }
      prodVal *= nums[end];
      res = max(res, end - start + 1);
      end++;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 1, 2, 1, 1, 1}, 5},
      {{2, 3, 4, 5, 6}, 3},
      {{1, 2, 3, 1, 4, 5, 1}, 5},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().maxLength(nums) == expected);
  }
}