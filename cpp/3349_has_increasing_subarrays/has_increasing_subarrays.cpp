#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool hasIncreasingSubarrays(vector<int>& nums, int k) {
    int n = nums.size();
    int cnt = 1, precnt = 0, ans = 0;
    for (int i = 1; i < n; ++i) {
      if (nums[i] > nums[i - 1]) {
        ++cnt;
      } else {
        precnt = cnt;
        cnt = 1;
      }
      ans = max(ans, min(precnt, cnt));
      ans = max(ans, cnt / 2);
    }
    return ans >= k;
  }
};

int main() {
  vector<tuple<vector<int>, int, bool>> tests{
      {{2, 5, 7, 8, 9, 2, 3, 4, 3, 1}, 3, true},
      {{1, 2, 3, 4, 4, 4, 4, 5, 6, 7}, 5, false},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().hasIncreasingSubarrays(nums, k) == ans);
  }
}