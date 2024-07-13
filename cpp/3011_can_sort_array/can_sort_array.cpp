
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canSortArray(vector<int>& nums) {
    int n = nums.size();
    int pre_max = 0;
    for (int i = 0; i < n;) {
      int mx = 0;
      int ones = __builtin_popcount(nums[i]);
      while (i < n && __builtin_popcount(nums[i]) == ones) {
        if (nums[i] < pre_max) {  // 无法排成有序的
          return false;
        }
        mx = max(mx, nums[i++]);  // 更新本组最大值
      }
      pre_max = mx;
    }
    return true;
  }
};

int main() {
  std::vector<std::tuple<vector<int>, bool>> tests{
      {{8, 4, 2, 30, 15}, true},
      {{1, 2, 3, 4, 5}, true},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().canSortArray(nums) == ans);
  }
}