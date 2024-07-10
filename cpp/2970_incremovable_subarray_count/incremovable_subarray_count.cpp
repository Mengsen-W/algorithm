#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int incremovableSubarrayCount(vector<int>& nums) {
    int n = nums.size();
    int res = 0;
    int l = 1;
    while (l < n && nums[l - 1] < nums[l]) {
      l++;
    }
    res += l + (l < n);
    for (int r = n - 2; r >= 0; r--) {
      while (l > 0 && nums[l - 1] >= nums[r + 1]) {
        l--;
      }
      res += l + (l <= r);
      if (nums[r] >= nums[r + 1]) {
        break;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 4}, 10},
      {{6, 5, 7, 8}, 7},
      {{8, 7, 6, 6}, 3},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().incremovableSubarrayCount(nums) == ans);
  }
}