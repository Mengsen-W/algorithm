#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumCost(vector<int> &nums) {
    int first = INT_MAX, second = INT_MAX;
    for (int i = 1; i < nums.size(); i++) {
      int x = nums[i];
      if (x < first) {
        second = first;
        first = x;
      } else if (x < second) {
        second = x;
      }
    }
    return nums[0] + first + second;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 12}, 6},
      {{5, 4, 3}, 12},
      {{10, 3, 1, 1}, 12},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().minimumCost(nums) == expected);
  }
}