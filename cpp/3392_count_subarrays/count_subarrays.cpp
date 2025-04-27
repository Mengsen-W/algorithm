#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countSubarrays(vector<int>& nums) {
    int n = nums.size();
    int ans = 0;
    for (int i = 1; i < n - 1; ++i) {
      if (nums[i] == (nums[i - 1] + nums[i + 1]) * 2) {
        ++ans;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 1, 4, 1}, 1},
      {{1, 1, 1}, 0},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().countSubarrays(nums) == ans);
  }
}