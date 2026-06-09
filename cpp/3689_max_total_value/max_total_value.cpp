#include <cassert>
#include <climits>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  long long maxTotalValue(vector<int>& nums, int k) {
    int m1 = INT_MAX, m2 = INT_MIN;
    for (int x : nums) {
      m1 = min(m1, x);
      m2 = max(m2, x);
    }
    return (long long)(m2 - m1) * k;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{1, 3, 2}, 2, 4},
      {{4, 2, 5, 1}, 3, 12},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().maxTotalValue(nums, k) == ans);
  }
  return 0;
}