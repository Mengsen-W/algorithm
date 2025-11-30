#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minNumberOperations(vector<int>& target) {
    int n = target.size();
    int ans = target[0];
    for (int i = 1; i < n; ++i) {
      ans += max(target[i] - target[i - 1], 0);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 2, 1}, 3},
      {{3, 1, 1, 2}, 4},
      {{3, 1, 5, 4, 2}, 7},
      {{1, 1, 1, 1}, 1},
  };

  for (auto [nums, expect] : tests) {
    assert(Solution().minNumberOperations(nums) == expect);
  }
}