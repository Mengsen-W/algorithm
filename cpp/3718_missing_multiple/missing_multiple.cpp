#include <cassert>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int missingMultiple(vector<int>& nums, int k) {
    unordered_set<int> seen(nums.begin(), nums.end());
    int ans = k;
    while (seen.count(ans)) {
      ans += k;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{8, 2, 3, 4, 6}, 2, 10},
      {{1, 4, 7, 10, 15}, 5, 5},
  };

  for (auto [nums, k, expected] : tests) {
    assert(Solution().missingMultiple(nums, k) == expected);
  }
}