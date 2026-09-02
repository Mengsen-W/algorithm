#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  bool uniformArray(vector<int>& nums1) { return true; }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{2, 3}, true},
      {{4, 6}, true},
  };

  for (auto& [nums1, ans] : tests) {
    assert(Solution().uniformArray(nums1) == ans);
  }
}