/*
 * @Date: 2023-08-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-13
 * @FilePath: /algorithm/cpp/88_merge/merge.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
    int p1 = m - 1, p2 = n - 1;
    int tail = m + n - 1;
    int cur;
    while (p1 >= 0 || p2 >= 0) {
      if (p1 == -1) {
        cur = nums2[p2--];
      } else if (p2 == -1) {
        cur = nums1[p1--];
      } else if (nums1[p1] > nums2[p2]) {
        cur = nums1[p1--];
      } else {
        cur = nums2[p2--];
      }
      nums1[tail--] = cur;
    }
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<int>, int, vector<int>>> test_cases{
      {{1, 2, 3, 0, 0, 0}, 3, {2, 5, 6}, 3, {1, 2, 2, 3, 5, 6}},
      {{1}, 1, {}, 0, {1}},
      {{0}, 0, {1}, 1, {1}},
  };
  for (auto& [nums1, m, nums2, n, expected] : test_cases) {
    Solution().merge(nums1, m, nums2, n);
    assert(nums1 == expected);
  }
}