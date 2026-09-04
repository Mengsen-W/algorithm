#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int firstStableIndex(vector<int>& nums, int k) {
    int n = nums.size();
    for (int i = 0; i < n; ++i) {
      int maxValue = nums[i], minValue = nums[i];
      for (int j = 0; j < i; ++j) {
        maxValue = max(maxValue, nums[j]);
      }
      for (int j = i + 1; j < n; ++j) {
        minValue = min(minValue, nums[j]);
      }
      if (maxValue - minValue <= k) {
        return i;
      }
    }
    return -1;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{5, 0, 1, 4}, 3, 3},
      {{3, 2, 1}, 1, -1},
      {{0}, 0, 0},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().firstStableIndex(nums, k) == ans);
  }
}