#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int triangleNumber(vector<int>& nums) {
    int n = nums.size();
    sort(nums.begin(), nums.end());
    int ans = 0;
    for (int i = 0; i < n; ++i) {
      int k = i;
      for (int j = i + 1; j < n; ++j) {
        while (k + 1 < n && nums[k + 1] < nums[i] + nums[j]) ++k;
        ans += max(k - j, 0);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 2, 3, 4}, 3},
      {{4, 2, 3, 4}, 4},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().triangleNumber(nums) == ans);
  }
  return 0;
}