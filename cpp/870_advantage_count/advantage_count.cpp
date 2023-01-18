/*
 * @Date: 2022-10-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-08
 * @FilePath: /algorithm/870_advantage_count/advantage_count.cpp
 */

#include <assert.h>

#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> advantageCount(vector<int>& nums1, vector<int>& nums2) {
    int n = nums1.size();
    vector<int> idx1(n), idx2(n);
    iota(idx1.begin(), idx1.end(), 0);
    iota(idx2.begin(), idx2.end(), 0);
    sort(idx1.begin(), idx1.end(), [&](int i, int j) { return nums1[i] < nums1[j]; });
    sort(idx2.begin(), idx2.end(), [&](int i, int j) { return nums2[i] < nums2[j]; });

    vector<int> ans(n);
    int left = 0, right = n - 1;
    for (int i = 0; i < n; ++i) {
      if (nums1[idx1[i]] > nums2[idx2[left]]) {
        ans[idx2[left]] = nums1[idx1[i]];
        ++left;
      } else {
        ans[idx2[right]] = nums1[idx1[i]];
        --right;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums1{2, 7, 11, 15}, nums2{1, 10, 4, 11}, ans{2, 11, 7, 15};
    assert(Solution().advantageCount(nums1, nums2) == ans);
  }

  {
    vector<int> nums1{12, 24, 8, 32}, nums2{13, 25, 32, 11}, ans{24, 32, 8, 12};
    assert(Solution().advantageCount(nums1, nums2) == ans);
  }
}