#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumUniqueSubarray(vector<int>& nums) {
    int n = nums.size();
    vector<int> psum(n + 1);
    unordered_map<int, int> cnt;
    int ans = 0, pre = 0;
    for (int i = 0; i < n; ++i) {
      psum[i + 1] = psum[i] + nums[i];
      pre = max(pre, cnt[nums[i]]);
      ans = max(ans, psum[i + 1] - psum[pre]);
      cnt[nums[i]] = i + 1;
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{4, 2, 4, 5, 6}, 17},
      {{5, 2, 1, 2, 5, 2, 1, 2, 5}, 8},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().maximumUniqueSubarray(nums) == ans);
  }
}