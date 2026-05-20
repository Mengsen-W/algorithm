#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int getCommon(vector<int>& nums1, vector<int>& nums2) {
    int i = 0;
    int j = 0;
    while (i < nums1.size() && j < nums2.size()) {
      if (nums1[i] == nums2[j]) {
        return nums1[i];
      }
      if (nums1[i] < nums2[j]) {
        i++;
      } else {
        j++;
      }
    }
    return -1;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{1, 2, 3}, {2, 4}, 2},
      {{1, 2, 3, 6}, {2, 3, 4, 5}, 2},
  };

  for (auto& [nums1, nums2, expect] : tests) {
    assert(Solution().getCommon(nums1, nums2) == expect);
  }
}