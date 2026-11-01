#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minMoves(vector<int> nums) {
    int minNum = *min_element(nums.begin(), nums.end());
    int res = 0;
    for (int num : nums) res += num - minNum;

    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3}, 3},
      {{1, 1, 1}, 0},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().minMoves(nums) == expected);
  }
}