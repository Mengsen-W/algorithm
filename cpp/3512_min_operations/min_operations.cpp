#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int>& nums, int k) { return accumulate(nums.begin(), nums.end(), 0) % k; }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{3, 9, 7}, 5, 4},
      {{4, 1, 3}, 4, 0},
      {{3, 2}, 6, 5},
  };

  for (auto& [nums, k, expect] : tests) {
    assert(Solution().minOperations(nums, k) == expect);
  }
}