#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int pivotIndex(vector<int> &nums) {
    int total = accumulate(nums.begin(), nums.end(), 0);
    int sum = 0;
    for (int i = 0; i < nums.size(); ++i) {
      if (2 * sum + nums[i] == total) {
        return i;
      }
      sum += nums[i];
    }
    return -1;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 7, 3, 6, 5, 6}, 3},
      {{1, 2, 3}, -1},
      {{2, 1, -1}, 0},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().pivotIndex(nums) == ans);
  }
}