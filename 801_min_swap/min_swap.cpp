/*
 * @Date: 2022-10-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-10
 * @FilePath: /algorithm/801_min_swap/min_swap.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minSwap(vector<int>& nums1, vector<int>& nums2) {
    int n = nums1.size();
    int a = 0, b = 1;
    for (int i = 1; i < n; i++) {
      int at = a, bt = b;
      a = b = n;
      if (nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1]) {
        a = min(a, at);
        b = min(b, bt + 1);
      }
      if (nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1]) {
        a = min(a, bt);
        b = min(b, at + 1);
      }
    }
    return min(a, b);
  }
};

int main() {
  {
    vector<int> nums1{1, 3, 5, 4};
    vector<int> nums2{1, 2, 3, 7};
    int ans = 1;
    assert(Solution().minSwap(nums1, nums2) == ans);
  }

  {
    vector<int> nums1{0, 3, 5, 8, 9};
    vector<int> nums2{2, 1, 4, 6, 9};
    int ans = 1;
    assert(Solution().minSwap(nums1, nums2) == ans);
  }
}