#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countMajoritySubarrays(vector<int>& nums, int target) {
    int n = nums.size();
    int ans = 0;
    for (int i = 0; i < n; ++i) {
      int cnt = 0;
      for (int j = i; j < n; ++j) {
        cnt += (nums[j] == target ? 1 : -1);
        if (cnt > 0) {
          ++ans;
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 2, 3}, 2, 5},
      {{1, 1, 1, 1}, 1, 10},
      {{1, 2, 3}, 4, 0},
  };

  for (auto& [nums, target, expected] : tests) {
    assert(Solution().countMajoritySubarrays(nums, target) == expected);
  }
}