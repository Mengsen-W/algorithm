
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int smallestRangeII(vector<int>& nums, int k) {
    sort(nums.begin(), nums.end());
    int mi = nums[0], ma = nums.back();
    int res = ma - mi;
    for (int i = 0; i < nums.size() - 1; i++) {
      int a = nums[i], b = nums[i + 1];
      res = min(res, max(ma - k, a + k) - min(mi + k, b - k));
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1}, 0, 0},
      {{0, 10}, 2, 6},
      {{1, 3, 6}, 3, 3},
  };

  for (auto &[nums, k, ans] : tests) {
    assert(Solution().smallestRangeII(nums, k) == ans);
  }
}