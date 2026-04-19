#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int maxDistance(vector<int>& nums1, vector<int>& nums2) {
    int n1 = nums1.size();
    int n2 = nums2.size();
    int i = 0;
    int res = 0;
    for (int j = 0; j < n2; ++j) {
      while (i < n1 && nums1[i] > nums2[j]) {
        ++i;
      }
      if (i < n1) {
        res = max(res, j - i);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{55, 30, 5, 4, 2}, {100, 20, 10, 10, 5}, 2},
      {{2, 2, 2}, {10, 10, 1}, 1},
      {{30, 29, 19, 5}, {25, 25, 25, 25, 25}, 2},
  };

  for (auto [nums1, nums2, expected] : tests) {
    assert(Solution().maxDistance(nums1, nums2) == expected);
  }
}