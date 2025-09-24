#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> intersect(vector<int>& nums1, vector<int>& nums2) {
    sort(nums1.begin(), nums1.end());
    sort(nums2.begin(), nums2.end());
    int length1 = nums1.size(), length2 = nums2.size();
    vector<int> intersection;
    int index1 = 0, index2 = 0;
    while (index1 < length1 && index2 < length2) {
      if (nums1[index1] < nums2[index2]) {
        index1++;
      } else if (nums1[index1] > nums2[index2]) {
        index2++;
      } else {
        intersection.push_back(nums1[index1]);
        index1++;
        index2++;
      }
    }
    return intersection;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, vector<int>>> tests{
      {{1, 2, 2, 1}, {2, 2}, {2, 2}},
      {{4, 9, 5}, {9, 4, 9, 8, 4}, {4, 9}},
  };
  

  for (auto &[nums1, nums2, ans] : tests) {
    assert(Solution().intersect(nums1, nums2) == ans);
  }
}