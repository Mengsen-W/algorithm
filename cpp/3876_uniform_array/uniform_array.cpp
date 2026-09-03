#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool uniformArray(vector<int>& nums1) {
    int mn = nums1[0];
    bool hasOdd = false;
    for (int v : nums1) {
      mn = std::min(v, mn);
      if ((v & 1) != 0) {
        hasOdd = true;
      }
    }
    if ((mn & 1) != 0) {
      return true;
    }
    return !hasOdd;
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{1, 4, 7}, true},
      {{2, 3}, false},
      {{4, 6}, true},
  };

  for (auto& [nums1, ans] : tests) {
    assert(Solution().uniformArray(nums1) == ans);
  }
}