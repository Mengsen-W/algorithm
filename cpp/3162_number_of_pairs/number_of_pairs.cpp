#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfPairs(vector<int>& nums1, vector<int>& nums2, int k) {
    int res = 0;
    for (int a : nums1) {
      for (int b : nums2) {
        if (a % (b * k) == 0) {
          res++;
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int, int>> tests{
      {{1, 3, 4}, {1, 3, 4}, 1, 5},
      {{1, 2, 4, 12}, {2, 4}, 3, 2},
  };

  for (auto& [nums1, nums2, k, ans] : tests) {
    assert(Solution().numberOfPairs(nums1, nums2, k) == ans);
  }
}