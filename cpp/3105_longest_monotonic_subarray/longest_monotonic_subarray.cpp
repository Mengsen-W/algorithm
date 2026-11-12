#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int longestMonotonicSubarray(vector<int> &nums) {
    int ans = 1;
    int i = 0;
    int n = nums.size();
    while (i < n - 1) {
      if (nums[i + 1] == nums[i]) {
        i++;
        continue;
      }
      int j = i;
      bool inc = nums[i + 1] > nums[i];
      i += 2;
      while (i < n && nums[i] != nums[i - 1] && (nums[i] > nums[i - 1]) == inc) {
        i++;
      }
      ans = max(ans, i - j);
      i--;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 4, 3, 3, 2}, 2},
      {{3, 3, 3, 3}, 1},
      {{3, 2, 1}, 3},
  };

  for (auto& [nums, expected] : tests) {
    assert(Solution().longestMonotonicSubarray(nums) == expected);
  }
}
