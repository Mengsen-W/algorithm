#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int addedInteger(vector<int>& nums1, vector<int>& nums2) {
    return (*max_element(nums2.begin(), nums2.end()) - *max_element(nums1.begin(), nums1.end()));
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{2, 6, 4}, {9, 7, 5}, 3},
      {{10}, {5}, -5},
      {{1, 1, 1, 1}, {1, 1, 1, 1}, 6},
  };

  for (auto& [nums1, nums2, ans] : tests) {
    assert(Solution().addedInteger(nums1, nums2) == ans);
  }
}