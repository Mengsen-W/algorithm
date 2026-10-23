#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int sortPermutation(vector<int>& nums) {
    int n = nums.size();
    int res = INT_MAX;
    for (int i = 0; i < n; i++) {
      if (nums[i] == i) {
        continue;
      }
      res &= nums[i];
    }
    return res == INT_MAX ? 0 : res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{0, 3, 2, 1}, 1},
      {{0, 1, 3, 2}, 2},
      {{3, 2, 1, 0}, 0},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().sortPermutation(nums) == ans);
  }
}