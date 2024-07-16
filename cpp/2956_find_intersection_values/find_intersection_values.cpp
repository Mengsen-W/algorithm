#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findIntersectionValues(vector<int>& nums1, vector<int>& nums2) {
    unordered_set<int> s1(nums1.begin(), nums1.end()), s2(nums2.begin(), nums2.end());
    vector<int> res(2);
    for (int x : nums1) {
      if (s2.count(x)) {
        res[0]++;
      }
    }
    for (int x : nums2) {
      if (s1.count(x)) {
        res[1]++;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, vector<int>>> tests{
      {{2, 3, 2}, {1, 2}, {2, 1}},
      {{4, 3, 2, 3, 1}, {2, 2, 5, 2, 3, 6}, {3, 4}},
      {{3, 4, 2, 3}, {1, 5}, {0, 0}},
  };

  for (auto& [nums1, nums2, ans] : tests) {
    assert(Solution().findIntersectionValues(nums1, nums2) == ans);
  }
}