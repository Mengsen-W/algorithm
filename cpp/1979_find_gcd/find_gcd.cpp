#include <algorithm>
#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findGCD(vector<int>& nums) {
    int mx = *max_element(nums.begin(), nums.end());
    int mn = *min_element(nums.begin(), nums.end());
    return gcd(mx, mn);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 5, 6, 9, 10}, 2},
      {{7, 5, 6, 8, 3}, 1},
      {{3, 3}, 3},
  };

  for (auto [nums, expect] : tests) {
    assert(Solution().findGCD(nums) == expect);
  }
}
