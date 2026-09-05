#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int firstStableIndex(vector<int>& nums, int k) {
    int n = nums.size();
    vector<int> minValue(n);
    minValue[n - 1] = nums[n - 1];
    for (int i = n - 2; i >= 0; --i) {
      minValue[i] = min(minValue[i + 1], nums[i]);
    }

    int maxValue = 0;
    for (int i = 0; i < n; ++i) {
      maxValue = max(maxValue, nums[i]);
      if (maxValue - minValue[i] <= k) {
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

  for (auto &[nums, k, ans] : tests) {
    assert(Solution().firstStableIndex(nums, k) == ans);
  }
}