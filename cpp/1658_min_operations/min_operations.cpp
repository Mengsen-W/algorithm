#include <cassert>
#include <numeric>
#include <vector>
#include <tuple>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int>& nums, int x) {
    int n = nums.size();
    int sum = accumulate(nums.begin(), nums.end(), 0);

    if (sum < x) {
      return -1;
    }

    int right = 0;
    int lsum = 0, rsum = sum;
    int ans = n + 1;

    for (int left = -1; left < n; ++left) {
      if (left != -1) {
        lsum += nums[left];
      }
      while (right < n && lsum + rsum > x) {
        rsum -= nums[right];
        ++right;
      }
      if (lsum + rsum == x) {
        ans = min(ans, (left + 1) + (n - right));
      }
    }

    return ans > n ? -1 : ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 1, 4, 2, 3}, 5, 2},
      {{5, 6, 7, 8, 9}, 4, -1},
      {{3, 2, 20, 1, 1, 3}, 10, 5},
  };
  for (auto& [nums, x, expected] : tests) {
    assert(Solution().minOperations(nums, x) == expected);
  }

  return 0;
}