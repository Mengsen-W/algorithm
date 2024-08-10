#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumAddedInteger(vector<int>& nums1, vector<int>& nums2) {
    int m = nums1.size(), n = nums2.size();
    sort(nums1.begin(), nums1.end());
    sort(nums2.begin(), nums2.end());
    for (int i : {2, 1, 0}) {
      int left = i + 1, right = 1;
      while (left < m && right < n) {
        if (nums1[left] - nums2[right] == nums1[i] - nums2[0]) {
          ++right;
        }
        ++left;
      }
      if (right == n) {
        return nums2[0] - nums1[i];
      }
    }
    // 本题不会有无解的情况
    return 0;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int>> tests{
      {{4, 20, 16, 12, 8}, {14, 18, 10}, -2},
      {{3, 5, 5, 3}, {7, 7}, 2},
  };
  
  for (auto &[nums1, nums2, ans] : tests) {
    assert(Solution().minimumAddedInteger(nums1, nums2) == ans);
  }
}
