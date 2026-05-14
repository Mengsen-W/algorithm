#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  bool isGood(vector<int>& nums) {
    int n = nums.size();
    vector<int> count(n, 0);
    for (int a : nums) {
      if (a >= n) {
        return false;
      }
      if (a < n - 1 && count[a] > 0) {
        return false;
      }
      if (a == n - 1 && count[a] > 1) {
        return false;
      }
      count[a]++;
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{2, 1, 3}, false},
      {{1, 3, 3, 2}, true},
      {{1, 1}, true},
      {{1, 2, 2, 3, 3, 4}, false},
  };

  for (auto& [nums, expected] : tests) {
    assert(Solution().isGood(nums) == expected);
  }
}