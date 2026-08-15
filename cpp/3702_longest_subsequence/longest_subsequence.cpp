#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int longestSubsequence(vector<int>& nums) {
    int n = nums.size();
    int totalXor = 0;
    bool allZero = true;

    for (int x : nums) {
      totalXor ^= x;
      if (x > 0) {
        allZero = false;
      }
    }

    if (totalXor > 0) {
      return n;
    }

    return allZero ? 0 : n - 1;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3}, 2},
      {{2, 3, 4}, 3},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().longestSubsequence(nums) == ans);
  }
}