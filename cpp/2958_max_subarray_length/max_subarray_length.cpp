#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxSubarrayLength(vector<int>& nums, int k) {
    int n = nums.size();
    unordered_map<int, int> occ;
    int right = -1, ans = 0;
    for (int left = 0; left < n; ++left) {
      if (left > 0) {
        --occ[nums[left - 1]];
      }
      while (right + 1 < n && occ[nums[right + 1]] < k) {
        ++right;
        ++occ[nums[right]];
      }
      ans = max(ans, right - left + 1);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 3, 1, 2, 3, 1, 2}, 2, 6},
      {{1, 2, 1, 2, 1, 2, 1, 2}, 1, 2},
      {{5, 5, 5, 5, 5, 5, 5}, 4, 4},
  };

  for (auto& [input, k, expected] : tests) {
    assert(Solution().maxSubarrayLength(input, k) == expected);
  }
}