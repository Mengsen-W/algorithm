#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestSubarray(vector<int>& nums) {
    int maxValue = nums[0];
    int ans = 1, cnt = 1;
    for (int i = 1; i < nums.size(); i++) {
      if (nums[i] > maxValue) {
        ans = cnt = 1;
        maxValue = nums[i];
      } else if (nums[i] < maxValue) {
        cnt = 0;
      } else if (nums[i] == maxValue) {
        cnt++;
      }
      ans = max(cnt, ans);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 3, 2, 2}, 2},
      {{1, 2, 3, 4}, 1},
  };

  for (auto& [test, expected] : tests) {
    assert(Solution().longestSubarray(test) == expected);
  }
}