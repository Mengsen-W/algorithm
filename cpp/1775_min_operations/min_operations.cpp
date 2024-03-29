/*
 * @Date: 2022-12-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-07
 * @FilePath: /algorithm/1775_min_operations/min_operations.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int help(vector<int>& h1, vector<int>& h2, int diff) {
    vector<int> h(7, 0);
    for (int i = 1; i < 7; ++i) {
      h[6 - i] += h1[i];
      h[i - 1] += h2[i];
    }
    int res = 0;
    for (int i = 5; i && diff > 0; --i) {
      int t = min((diff + i - 1) / i, h[i]);
      res += t;
      diff -= t * i;
    }
    return res;
  }

  int minOperations(vector<int>& nums1, vector<int>& nums2) {
    int n = nums1.size(), m = nums2.size();
    if (6 * n < m || 6 * m < n) {
      return -1;
    }
    vector<int> cnt1(7, 0), cnt2(7, 0);
    int diff = 0;
    for (auto& i : nums1) {
      ++cnt1[i];
      diff += i;
    }
    for (auto& i : nums2) {
      ++cnt2[i];
      diff -= i;
    }
    if (!diff) {
      return 0;
    }
    if (diff > 0) {
      return help(cnt2, cnt1, diff);
    }
    return help(cnt1, cnt2, -diff);
  }
};

int main() {
  {
    vector<int> nums1{1, 2, 3, 4, 5, 6};
    vector<int> nums2{1, 1, 2, 2, 2, 2};
    int ans = 3;
    assert(Solution().minOperations(nums1, nums2) == ans);
  }

  {
    vector<int> nums1{1, 1, 1, 1, 1, 1, 1};
    vector<int> nums2{6};
    int ans = -1;
    assert(Solution().minOperations(nums1, nums2) == ans);
  }

  {
    vector<int> nums1{6, 6};
    vector<int> nums2{1};
    int ans = 3;
    assert(Solution().minOperations(nums1, nums2) == ans);
  }
}
