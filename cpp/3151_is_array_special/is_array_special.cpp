#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isArraySpecial(vector<int>& nums) {
    int n = nums.size();
    for (int i = 1; i < n; ++i) {
      if (nums[i - 1] % 2 == nums[i] % 2) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{1}, true},
      {{2, 1, 4}, true},
      {{4, 3, 1, 6}, false},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().isArraySpecial(nums) == ans);
  }
}