#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool check(vector<int>& nums) {
    int n = nums.size(), x = 0;
    for (int i = 1; i < n; ++i) {
      if (nums[i] < nums[i - 1]) {
        x = i;
        break;
      }
    }
    if (x == 0) {
      return true;
    }
    for (int i = x + 1; i < n; ++i) {
      if (nums[i] < nums[i - 1]) {
        return false;
      }
    }
    return nums[0] >= nums[n - 1];
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{3, 4, 5, 1, 2}, true},
      {{2, 1, 3, 4}, false},
      {{1, 2, 3}, true},
  };
  for (auto& [nums, expected] : tests) {
    assert(Solution().check(nums) == expected);
  }
}