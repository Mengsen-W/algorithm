#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  long long incremovableSubarrayCount(vector<int>& nums) {
    long long ans = 0;
    int len = nums.size();
    int l = 0;
    while (l < len - 1) {
      if (nums[l] >= nums[l + 1]) {
        break;
      }
      l++;
    }
    if (l == len - 1) {
      return 1LL * len * (len + 1) / 2;
    }

    ans += l + 2;
    for (int r = len - 1; r > 0; r--) {
      if (r < len - 1 && nums[r] >= nums[r + 1]) {
        break;
      }

      while (l >= 0 && nums[l] >= nums[r]) {
        l--;
      }
      ans += l + 2;
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{1, 2, 3, 4}, 10},
      {{6, 5, 7, 8}, 7},
      {{8, 7, 6, 6}, 3},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().incremovableSubarrayCount(nums) == ans);
  }
}
