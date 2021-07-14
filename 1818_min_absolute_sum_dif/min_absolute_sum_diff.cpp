/*
 * @Date: 2021-07-14 08:27:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-14 08:42:47
 */

#include <algorithm>
#include <cassert>
#include <vector>
using namespace std;

static constexpr int mod = 1'000'000'007;

int minAbsoluteSumDiff(vector<int>& nums1, vector<int>& nums2) {
  vector<int> rec(nums1);
  sort(rec.begin(), rec.end());
  int sum = 0, maxn = 0;
  int n = nums1.size();
  for (int i = 0; i < n; i++) {
    int diff = abs(nums1[i] - nums2[i]);
    sum = (sum + diff) % mod;
    int j = lower_bound(rec.begin(), rec.end(), nums2[i]) - rec.begin();
    if (j < n) {
      maxn = max(maxn, diff - (rec[j] - nums2[i]));
    }
    if (j > 0) {
      maxn = max(maxn, diff - (nums2[i] - rec[j - 1]));
    }
  }
  return (sum - maxn + mod) % mod;
}

int main() {
  {
    vector<int> nums1{1, 7, 5};
    vector<int> nums2{2, 3, 5};
    assert(minAbsoluteSumDiff(nums1, nums2) == 3);
  }
  {
    vector<int> nums1{2, 4, 6, 8, 10};
    vector<int> nums2{2, 4, 6, 8, 10};
    assert(minAbsoluteSumDiff(nums1, nums2) == 0);
  }
  {
    vector<int> nums1{1, 10, 4, 4, 2, 7};
    vector<int> nums2{9, 3, 5, 1, 7, 4};
    assert(minAbsoluteSumDiff(nums1, nums2) == 20);
  }
}