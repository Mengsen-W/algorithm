#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int semiOrderedPermutation(vector<int>& nums) {
    auto [first, last] = minmax_element(nums.begin(), nums.end());
    return first + nums.size() - 1 - last - (last < first);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 1, 4, 3}, 2},
      {{2, 4, 1, 3}, 3},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().semiOrderedPermutation(nums) == ans);
  }
}