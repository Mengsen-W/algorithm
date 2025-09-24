#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestSubarray(vector<int>& nums) {
    int ans = 0;
    int p0 = 0, p1 = 0;
    for (int num : nums) {
      if (num == 0) {
        p1 = p0;
        p0 = 0;
      } else {
        ++p0;
        ++p1;
      }
      ans = max(ans, p1);
    }
    if (ans == nums.size()) {
      --ans;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 1, 0, 1}, 3},
      {{0, 1, 1, 1, 0, 1, 1, 0, 1}, 5},
      {{1, 1, 1}, 2},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().longestSubarray(nums) == ans);
  }
}